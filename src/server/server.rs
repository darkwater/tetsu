use anyhow::{Context as _, Result};
use tarpc::context::Context;

use super::interface::{Error, TetsuServer};
use crate::anidb::records::{Anime, Episode};

#[derive(Clone)]
pub struct Server;

#[tarpc::server]
impl TetsuServer for Server {
    async fn anime(self, _: Context) -> Result<Vec<Anime>, Error> {
        let db = crate::DB.get().await;

        let mut anime = sqlx::query!("SELECT json FROM anime")
            .fetch_all(db)
            .await
            .context("Database query failed")?
            .into_iter()
            .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
            .collect::<Result<Vec<Anime>>>()?;

        anime.sort_by(|a, b| a.romaji_name.cmp(&b.romaji_name));

        Ok(anime)
    }

    async fn episodes(self, _: Context, aid: u32) -> Result<Vec<Episode>, Error> {
        let db = crate::DB.get().await;

        let mut episodes = sqlx::query!("SELECT json FROM episodes WHERE aid = ?", aid)
            .fetch_all(db)
            .await
            .context("Database query failed")?
            .into_iter()
            .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
            .collect::<Result<Vec<Episode>>>()?;

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

        Ok(episodes)
    }
}
