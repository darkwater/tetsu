use std::{num::NonZeroU64, sync::Arc};

use anyhow::Context;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use tokio::sync::RwLock;

use self::platform_links::PlatformLinks;
use super::Result;
use crate::anidb::{
    records::{Anime, Episode, File},
    Anidb,
};

pub mod mpv;
pub mod platform_links;
pub mod settings;

pub mod proxy {
    pub mod animebytes;
}

#[derive(Serialize)]
pub struct AnimeWithLinks {
    #[serde(flatten)]
    anime: Anime,
    links: platform_links::PlatformLinks,
}

pub async fn all_anime() -> Result<Json<Vec<AnimeWithLinks>>> {
    let db = crate::DB.get().await;

    let mut anime = sqlx::query!(
        "SELECT a.json, pl.*
         FROM indexed_files if
         INNER JOIN files f
            ON if.fid = f.fid
         INNER JOIN anime a
            ON f.aid = a.aid
         INNER JOIN platform_links pl
            ON a.aid = pl.anidb_id
         GROUP BY a.aid",
    )
    .fetch_all(db)
    .await
    .context("Database query failed")?
    .into_iter()
    .map(|row| {
        Ok(AnimeWithLinks {
            anime: serde_json::from_str(&row.json).context("Invalid record in database")?,
            links: PlatformLinks {
                animebytes_id: NonZeroU64::new(row.animebytes_id as u64),
                anidb_id: NonZeroU64::new(row.anidb_id as u64),
                ann_id: NonZeroU64::new(row.ann_id as u64),
                anilist_id: NonZeroU64::new(row.anilist_id as u64),
                mal_id: NonZeroU64::new(row.mal_id as u64),
            },
        })
    })
    .collect::<anyhow::Result<Vec<AnimeWithLinks>>>()?;

    anime.sort_by(|a, b| a.anime.romaji_name.cmp(&b.anime.romaji_name));

    Ok(Json(anime))
}

pub async fn anime(
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

pub async fn anime_episodes(Path(aid): Path<u32>) -> Result<Json<Vec<Episode>>> {
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
pub struct WrappedFile {
    #[serde(flatten)]
    info: File,
    path: String,
}

pub async fn anime_files(Path(aid): Path<u32>) -> Result<Json<Vec<WrappedFile>>> {
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
