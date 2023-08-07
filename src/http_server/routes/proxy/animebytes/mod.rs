use anyhow::Context;
use axum::extract::Path;

use crate::http_server::Result;

mod search;
pub use search::search;

pub async fn torrent(Path(id): Path<i64>) -> Result<String> {
    let group = sqlx::query!(
        "SELECT g.data
         FROM animebytes_torrents t
         INNER JOIN animebytes_groups g
            ON t.group_id = g.id
         WHERE t.torrent_id = $1",
        id
    )
    .fetch_optional(crate::DB.get().await)
    .await
    .context("Database query failed")?
    .map(|row| row.data);

    Ok(group.unwrap_or_else(|| "null".to_string()))
}

pub async fn group(Path(id): Path<i64>) -> Result<String> {
    let group = sqlx::query!(
        "SELECT data
         FROM animebytes_groups
         WHERE id = $1",
        id
    )
    .fetch_optional(crate::DB.get().await)
    .await
    .context("Database query failed")?
    .map(|row| row.data);

    Ok(group.unwrap_or_else(|| "null".to_string()))
}
