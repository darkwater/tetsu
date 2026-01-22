use std::{str::FromStr, time::Duration};

use anyhow::{bail, Context, Result};
use tokio::{
    net::UdpSocket,
    time::{timeout_at, Instant},
};

use super::{
    command_builder::CommandBuilder,
    records::{Anime, Episode, File, Group},
    response::{codes::ResponseCode, Response},
};
use crate::db::settings;

pub struct Session {
    key: Option<String>,
    next_call: Instant,
    socket: UdpSocket,
}

impl Session {
    pub async fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:16835")
            .await
            .context("Failed to bind UDP socket")?;

        socket.connect("api.anidb.net:9000").await?;

        Ok(Self {
            key: settings::anidb::session_key().await?,
            next_call: Instant::now(),
            socket,
        })
    }

    pub fn requires_auth(&self, cmd: &CommandBuilder) -> bool {
        !["PING", "ENCRYPT", "ENCODING", "AUTH", "VERSION"].contains(&cmd.name())
    }

    pub async fn session_key(&mut self) -> Result<&str> {
        if let Some(ref key) = self.key {
            return Ok(key);
        }

        self.login().await
    }

    pub async fn request(&mut self, mut cmd: CommandBuilder) -> Result<Response> {
        if self.requires_auth(&cmd) {
            cmd = cmd.arg("s", self.session_key().await?);
        }

        let res = self.request_inner(&cmd.to_string()).await?;

        match res.code {
            ResponseCode::InvalidSession | ResponseCode::LoginFirst => {
                self.key = None;
                let newkey = self.login().await?;
                cmd = cmd.arg("s", newkey);
                self.request_inner(&cmd.to_string()).await
            }
            _ => Ok(res),
        }
    }

    pub async fn request_inner(&mut self, cmd: &str) -> Result<Response> {
        tokio::time::sleep_until(self.next_call).await;

        let mut buf = [0; 1400]; // 1400 is AniDB's default and maximum MTU

        let mut retries = 3;

        let bytes = loop {
            self.next_call = Instant::now() + Duration::from_secs(2);

            for line in cmd.to_string().lines() {
                log::trace!("-> {}", line);
            }

            self.socket.send(cmd.as_bytes()).await?;

            match timeout_at(self.next_call, self.socket.recv(&mut buf)).await {
                Ok(Ok(read)) => break buf[..read].to_owned(),
                Ok(Err(e)) => bail!("Failed to read response: {}", e),
                Err(_) => {
                    if retries == 0 {
                        bail!("Timed out waiting for response");
                    }

                    retries -= 1;
                }
            }
        };

        let s = String::from_utf8(bytes)?;

        for line in s.lines() {
            log::trace!("<- {}", line);
        }

        Response::from_str(&s).context(format!("Failed to parse response:\n{s}"))
    }

    pub async fn login(&mut self) -> Result<&str> {
        let cmd = CommandBuilder::new("AUTH")
            .arg(
                "user",
                settings::anidb::username()
                    .await
                    .context("Failed to read username")?
                    .context("Username unset")?,
            )
            .arg(
                "pass",
                settings::anidb::password()
                    .await
                    .context("Failed to read password")?
                    .context("Password unset")?,
            )
            .arg("protover", 3)
            .arg("client", "tetsu")
            .arg("clientver", 1)
            .arg("enc", "UTF8");

        let res = self.request_inner(&cmd.to_string()).await?;

        match res.code {
            ResponseCode::LoginAccepted => (),
            ResponseCode::LoginAcceptedNewVersion => {
                log::warn!("New version of Tetsu available")
            }
            _ => bail!("Login failed: {}", res.message),
        }

        self.key = Some(res.data().unwrap().to_string());

        settings::anidb::set_session_key(self.key.clone().unwrap()).await?;

        Ok(self.key.as_ref().unwrap())
    }

    async fn file_inner(&mut self, cmd: CommandBuilder) -> Result<Option<File>> {
        let cmd = cmd.arg("fmask", "71c2fef800").arg("amask", "00000000");

        let res = self.request(cmd).await?;

        if res.code == ResponseCode::NoSuchFile {
            return Ok(None);
        }

        if res.code != ResponseCode::File {
            bail!("Unexpected response code: {:?}", res.code);
        }

        let item = res.records_as::<File>().next();

        match item.transpose().map_err(Into::into) {
            Ok(Some(file)) => {
                let json = serde_json::to_string(&file)?;

                sqlx::query!(
                    "INSERT INTO files (fid, aid, eid, gid, size, ed2k, json)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT (fid) DO UPDATE SET
                        aid = $2,
                        eid = $3,
                        gid = $4,
                        size = $5,
                        ed2k = $6,
                        json = $7",
                    file.fid,
                    file.aid,
                    file.eid,
                    file.gid,
                    file.size,
                    file.ed2k,
                    json,
                )
                .execute(crate::DB.get().await)
                .await?;

                Ok(Some(file))
            }
            v => v,
        }
    }

    pub async fn file_by_ed2k(&mut self, size: i64, hash: &str) -> Result<Option<File>> {
        let cmd = CommandBuilder::new("FILE")
            .arg("size", size)
            .arg("ed2k", hash);

        self.file_inner(cmd).await
    }

    pub async fn file_by_fid(&mut self, fid: u32) -> Result<Option<File>> {
        let cmd = CommandBuilder::new("FILE").arg("fid", fid);

        self.file_inner(cmd).await
    }

    pub async fn anime_by_aid(&mut self, aid: u32) -> Result<Option<Anime>> {
        let cmd = CommandBuilder::new("ANIME")
            .arg("aid", aid)
            .arg("amask", "fce8ba014080f8");

        let res = self.request(cmd).await?;

        if res.code == ResponseCode::NoSuchAnime {
            return Ok(None);
        }

        if res.code != ResponseCode::Anime {
            bail!("Unexpected response code: {:?}", res.code);
        }

        let item = res.records_as::<Anime>().next();

        match item.transpose().map_err(Into::into) {
            Ok(Some(anime)) => {
                let json = serde_json::to_string(&anime)?;

                sqlx::query!(
                    "INSERT INTO anime (aid, json)
                    VALUES ($1, $2)
                    ON CONFLICT (aid) DO UPDATE SET
                        json = $2",
                    anime.aid,
                    json,
                )
                .execute(crate::DB.get().await)
                .await?;

                let link = sqlx::query!(
                    "SELECT id FROM platform_links
                    WHERE anidb_id = $1 OR ann_id = $2",
                    anime.aid,
                    anime.ann_id,
                )
                .fetch_optional(crate::DB.get().await)
                .await?;

                if let Some(link) = link {
                    sqlx::query!(
                        "UPDATE platform_links
                        SET anidb_id = $1, ann_id = $2
                        WHERE id = $3",
                        anime.aid,
                        anime.ann_id,
                        link.id,
                    )
                    .execute(crate::DB.get().await)
                    .await?;
                } else {
                    sqlx::query!(
                        "INSERT INTO platform_links (anidb_id, ann_id)
                        VALUES ($1, $2)",
                        anime.aid,
                        anime.ann_id,
                    )
                    .execute(crate::DB.get().await)
                    .await?;
                }

                Ok(Some(anime))
            }
            v => v,
        }
    }

    pub async fn episode_by_eid(&mut self, eid: u32) -> Result<Option<Episode>> {
        let cmd = CommandBuilder::new("EPISODE").arg("eid", eid);

        let res = self.request(cmd).await?;

        if res.code == ResponseCode::NoSuchEpisode {
            return Ok(None);
        }

        if res.code != ResponseCode::Episode {
            bail!("Unexpected response code: {:?}", res.code);
        }

        let item = res.records_as::<Episode>().next();

        match item.transpose().map_err(Into::into) {
            Ok(Some(episode)) => {
                let json = serde_json::to_string(&episode)?;

                sqlx::query!(
                    "INSERT INTO episodes (eid, aid, json)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (eid) DO UPDATE SET
                        aid = $2,
                        json = $3",
                    episode.eid,
                    episode.aid,
                    json,
                )
                .execute(crate::DB.get().await)
                .await?;

                Ok(Some(episode))
            }
            v => v,
        }
    }

    pub async fn group_by_gid(&mut self, gid: u32) -> Result<Option<Group>> {
        let cmd = CommandBuilder::new("GROUP").arg("gid", gid);

        let res = self.request(cmd).await?;

        if res.code == ResponseCode::NoSuchGroup {
            return Ok(None);
        }

        if res.code != ResponseCode::Group {
            bail!("Unexpected response code: {:?}", res.code);
        }

        let item = res.records_as::<Group>().next();

        match item.transpose().map_err(Into::into) {
            Ok(Some(group)) => {
                let json = serde_json::to_string(&group)?;

                sqlx::query!(
                    "INSERT INTO groups (gid, json)
                    VALUES ($1, $2)
                    ON CONFLICT (gid) DO UPDATE SET
                        json = $2",
                    group.gid,
                    json,
                )
                .execute(crate::DB.get().await)
                .await?;

                Ok(Some(group))
            }
            v => v,
        }
    }
}
