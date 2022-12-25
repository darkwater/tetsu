use std::{str::FromStr, time::Duration};

use anyhow::{bail, Context, Result};
use tokio::{net::UdpSocket, time::Instant};

use super::{
    command_builder::CommandBuilder,
    models::File,
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
            ResponseCode::InvalidSession => {
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

        self.next_call = Instant::now() + Duration::from_secs(2);

        for line in cmd.to_string().lines() {
            log::trace!("-> {}", line);
        }

        self.socket.send(cmd.to_string().as_bytes()).await?;

        let mut buf = [0; 1400]; // 1400 is AniDB's default and maximum MTU
        let read = self.socket.recv(&mut buf).await?;
        let bytes = buf[..read].to_owned();
        let s = String::from_utf8(bytes)?;

        for line in s.lines() {
            log::trace!("<- {}", line);
        }

        Response::from_str(&s)
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

    pub async fn file_by_ed2k(&mut self, size: i64, hash: &str) -> Result<Option<File>> {
        let cmd = CommandBuilder::new("FILE")
            .arg("size", size)
            .arg("ed2k", hash)
            .arg("fmask", "71c2fef800")
            .arg("amask", "00000000");

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
}
