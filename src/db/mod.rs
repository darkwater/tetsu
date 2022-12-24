use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};

use crate::CONFIG;

pub async fn init() -> Result<SqlitePool> {
    let config = CONFIG.read().await;
    let mut path = config.db_path.clone();

    if !path.is_absolute() && path.starts_with("~") {
        let home = std::env::var("HOME").context("Cannot expand ~: $HOME is not set")?;
        path = PathBuf::from(home);
        path.push(config.db_path.strip_prefix("~").unwrap());
    }

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal),
    )
    .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
