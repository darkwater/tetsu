use anyhow::{Context, Result};

use self::{
    records::{Anime, Episode, File},
    session::Session,
};
use crate::db::settings;

mod command_builder;
pub mod records;
mod response;
mod session;

pub async fn login() -> Result<()> {
    let username = dialoguer::Input::new()
        .with_prompt("Username")
        .interact()
        .context("Failed to read username")?;

    let password = dialoguer::Password::new()
        .with_prompt("Password")
        .interact()
        .context("Failed to read password")?;

    settings::anidb::set_username(username).await?;
    settings::anidb::set_password(password).await?;

    Ok(())
}

pub struct Anidb {
    session: Option<Session>,
}

impl Anidb {
    pub fn new() -> Self {
        Self { session: None }
    }

    async fn session(&mut self) -> Result<&mut Session> {
        if let Some(ref mut session) = self.session {
            return Ok(session);
        }

        self.session = Some(Session::new().await?);
        Ok(self.session.as_mut().unwrap())
    }

    pub async fn file_by_ed2k(&mut self, size: i64, hash: &str) -> Result<Option<File>> {
        let cache =
            sqlx::query!("SELECT json FROM files WHERE size = $1 AND ed2k = $2", size, hash)
                .fetch_optional(crate::DB.get().await)
                .await?;

        if let Some(file) = cache {
            return Ok(Some(
                serde_json::from_str(&file.json).context("Invalid record in database")?,
            ));
        }

        self.session().await?.file_by_ed2k(size, hash).await
    }

    pub async fn file_by_fid(&mut self, fid: u32) -> Result<Option<File>> {
        let cache = sqlx::query!("SELECT json FROM files WHERE fid = $1", fid)
            .fetch_optional(crate::DB.get().await)
            .await?;

        if let Some(file) = cache {
            return Ok(Some(
                serde_json::from_str(&file.json).context("Invalid record in database")?,
            ));
        }

        self.session().await?.file_by_fid(fid).await
    }

    pub async fn anime_by_aid(&mut self, aid: u32) -> Result<Option<Anime>> {
        let cache = sqlx::query!("SELECT json FROM anime WHERE aid = $1", aid)
            .fetch_optional(crate::DB.get().await)
            .await?;

        if let Some(anime) = cache {
            return Ok(Some(
                serde_json::from_str(&anime.json).context("Invalid record in database")?,
            ));
        }

        self.session().await?.anime_by_aid(aid).await
    }

    pub async fn episode_by_eid(&mut self, eid: u32) -> Result<Option<Episode>> {
        let cache = sqlx::query!("SELECT json FROM episodes WHERE eid = $1", eid)
            .fetch_optional(crate::DB.get().await)
            .await?;

        if let Some(episode) = cache {
            return Ok(Some(
                serde_json::from_str(&episode.json).context("Invalid record in database")?,
            ));
        }

        self.session().await?.episode_by_eid(eid).await
    }

    pub async fn group_by_gid(&mut self, gid: u32) -> Result<Option<records::Group>> {
        let cache = sqlx::query!("SELECT json FROM groups WHERE gid = $1", gid)
            .fetch_optional(crate::DB.get().await)
            .await?;

        if let Some(group) = cache {
            return Ok(Some(
                serde_json::from_str(&group.json).context("Invalid record in database")?,
            ));
        }

        self.session().await?.group_by_gid(gid).await
    }
}
