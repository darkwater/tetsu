use anyhow::{Context, Result};

use self::{models::File, session::Session};
use crate::db::settings;

mod command_builder;
mod models;
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
}
