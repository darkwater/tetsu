use std::sync::Arc;

use anyhow::Context as _;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tokio::sync::RwLock;

use self::mpv::mpv_upgrade;
use crate::anidb::{
    records::{Anime, Episode, File},
    Anidb,
};

mod error;
mod mpv;

type Result<T> = std::result::Result<T, error::AppError>;

pub async fn run() -> anyhow::Result<()> {
    let anidb = Arc::new(RwLock::new(Anidb::new()));

    let app = Router::new()
        .route("/anime", get(all_anime))
        .route("/anime/:aid", get(anime))
        .route("/anime/:aid/episodes", get(anime_episodes))
        .route("/anime/:aid/files", get(anime_files))
        .route("/mpv", get(mpv_upgrade))
        .with_state(anidb);

    axum::Server::bind(&"127.0.0.1:5352".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .context("Server error")
}

#[derive(Clone)]
pub struct Server;

async fn all_anime() -> Result<Json<Vec<Anime>>> {
    let db = crate::DB.get().await;

    let mut anime = sqlx::query!(
        "SELECT a.json
         FROM indexed_files if
         INNER JOIN files f
            ON if.fid = f.fid
         INNER JOIN anime a
            ON f.aid = a.aid
         GROUP BY a.aid",
    )
    .fetch_all(db)
    .await
    .context("Database query failed")?
    .into_iter()
    .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
    .collect::<anyhow::Result<Vec<Anime>>>()?;

    anime.sort_by(|a, b| a.romaji_name.cmp(&b.romaji_name));

    Ok(Json(anime))
}

async fn anime(
    Path(aid): Path<u32>,
    State(state): State<Arc<RwLock<Anidb>>>,
) -> Result<Json<Anime>> {
    Ok(Json(
        state
            .write()
            .await
            .anime_by_aid(aid)
            .await
            .context("Couldn't fetch from AniDB")?
            .context("Not found on AniDB")?,
    ))
}

async fn anime_episodes(Path(aid): Path<u32>) -> Result<Json<Vec<Episode>>> {
    let db = crate::DB.get().await;

    let mut episodes = sqlx::query!("SELECT json FROM episodes WHERE aid = ?", aid)
        .fetch_all(db)
        .await
        .context("Database query failed")?
        .into_iter()
        .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
        .collect::<anyhow::Result<Vec<Episode>>>()?;

    episodes.sort_by_cached_key(|Episode { epno, .. }| {
        let Some(idx) = epno.bytes().position(|b| b.is_ascii_digit()) else {
            return (epno.clone(), 0);
        };

        let (alpha, num) = epno.split_at(idx);
        let Ok(num) = num.parse() else {
            return (epno.clone(), 0);
        };

        (alpha.to_string(), num)
    });

    Ok(Json(episodes))
}

#[derive(Serialize)]
struct WrappedFile {
    #[serde(flatten)]
    info: File,
    path: String,
}

async fn anime_files(Path(aid): Path<u32>) -> Result<Json<Vec<WrappedFile>>> {
    let db = crate::DB.get().await;

    let mut files = sqlx::query!(
        "SELECT if.path, f.json
         FROM indexed_files if
         INNER JOIN files f
            ON if.fid = f.fid
         WHERE f.aid = ?",
        aid
    )
    .fetch_all(db)
    .await
    .context("Database query failed")?
    .into_iter()
    .map(|row| {
        Ok(WrappedFile {
            info: serde_json::from_str(&row.json).context("Invalid record in database")?,
            path: row.path,
        })
    })
    .collect::<anyhow::Result<Vec<WrappedFile>>>()?;

    files.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(Json(files))
}
