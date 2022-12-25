use std::path::PathBuf;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    SqlitePool,
};

use crate::CONFIG;

pub mod settings;

pub async fn init() -> SqlitePool {
    let config = CONFIG.read().await;
    let mut path = config.db_path.clone();

    if !path.is_absolute() && path.starts_with("~") {
        let home = std::env::var("HOME").expect("Cannot expand ~: $HOME is not set");
        path = PathBuf::from(home);
        path.push(config.db_path.strip_prefix("~").unwrap());
    }

    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal),
    )
    .await
    .expect("Failed to open database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
