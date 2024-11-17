use std::{num::NonZeroU64, sync::Arc};

use anyhow::Context;
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
pub struct AnimeWithExtras {
    #[serde(flatten)]
    anime: Anime,
    links: platform_links::PlatformLinks,
    watch_progress: Option<WatchProgress>,
}

#[derive(Serialize)]
pub struct WatchProgress {
    last_eid: i64,
    episode_progress: f64,
    anime_progress: f64,
    last_updated: DateTime<Utc>,
}

pub async fn all_anime() -> Result<Json<Vec<AnimeWithExtras>>> {
    let db = crate::DB.get().await;

    let mut anime = sqlx::query!(
        "SELECT a.json, pl.*, wp.*
         FROM indexed_files if
         INNER JOIN files f
            ON if.fid = f.fid
         INNER JOIN anime a
            ON f.aid = a.aid
         INNER JOIN platform_links pl
            ON a.aid = pl.anidb_id
         LEFT OUTER JOIN watch_progress wp
            ON a.aid = wp.aid
         GROUP BY a.aid",
    )
    .fetch_all(db)
    .await
    .context("Database query failed")?
    .into_iter()
    .map(|row| {
        Ok(AnimeWithExtras {
            anime: serde_json::from_str(&row.json).context("Invalid record in database")?,
            links: PlatformLinks {
                animebytes_id: NonZeroU64::new(row.animebytes_id.unwrap_or_default() as u64),
                anidb_id: NonZeroU64::new(row.anidb_id.unwrap_or_default() as u64),
                ann_id: NonZeroU64::new(row.ann_id.unwrap_or_default() as u64),
                anilist_id: NonZeroU64::new(row.anilist_id.unwrap_or_default() as u64),
                mal_id: NonZeroU64::new(row.mal_id.unwrap_or_default() as u64),
            },
            watch_progress: row.last_eid.map(|last_eid| WatchProgress {
                last_eid,
                episode_progress: row.episode_progress.unwrap_or_default(),
                anime_progress: row.anime_progress.unwrap_or_default(),
                last_updated: DateTime::from_timestamp(row.last_updated.unwrap_or_default(), 0)
                    .unwrap_or_default(),
            }),
        })
    })
    .collect::<anyhow::Result<Vec<AnimeWithExtras>>>()?;

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

#[derive(Deserialize)]
pub struct ReportProgress {
    filepath: String,
    progress: f32,
}

pub async fn report_progress(
    Json(ReportProgress { filepath, progress }): Json<ReportProgress>,
) -> Result<()> {
    let db = crate::DB.get().await;

    let row = sqlx::query!(
        "SELECT a.aid, a.json as ajson, e.eid, e.json as ejson
         FROM indexed_files if
         INNER JOIN files f
            ON if.fid = f.fid
         INNER JOIN episodes e
            ON f.eid = e.eid
         INNER JOIN anime a
            ON f.aid = a.aid
         WHERE if.path = ?",
        filepath
    )
    .fetch_one(db)
    .await
    .context("Database query failed")?;

    let anime = serde_json::from_str::<Anime>(&row.ajson).context("Invalid record in database")?;
    let episode =
        serde_json::from_str::<Episode>(&row.ejson).context("Invalid record in database")?;

    // could be like C02, ignore those
    let epno = episode
        .epno
        .parse::<u32>()
        .context("Couldn't parse episode number")? as f32;
    let episodes = anime.episodes as f32;
    let anime_progress = (epno - 1.) / episodes + progress / episodes;

    dbg!((epno, episodes, progress, anime_progress));

    let now = Utc::now().timestamp();

    sqlx::query!(
        "INSERT INTO watch_progress (aid, last_eid, episode_progress, anime_progress, last_updated)
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT (aid) DO UPDATE SET last_eid = ?, episode_progress = ?, anime_progress = ?, last_updated = ?",
        row.aid,
        row.eid,
        progress,
        anime_progress,
        now,
        row.eid,
        progress,
        anime_progress,
        now,
    )
    .execute(db)
    .await?;

    Ok(())
}
