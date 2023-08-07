use std::{collections::HashMap, num::NonZeroU64};

use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::http_server::Result;

#[derive(FromRow)]
pub struct PlatformLinksRow {
    id: i64,
    animebytes_id: i64,
    anidb_id: i64,
    ann_id: i64,
    anilist_id: i64,
    mal_id: i64,
}

#[derive(Serialize)]
pub struct PlatformLinks {
    pub animebytes_id: Option<NonZeroU64>,
    pub anidb_id: Option<NonZeroU64>,
    pub ann_id: Option<NonZeroU64>,
    pub anilist_id: Option<NonZeroU64>,
    pub mal_id: Option<NonZeroU64>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformLinksParam {
    AnimebytesId(i64),
    AnidbId(i64),
    AnnId(i64),
    AnilistId(i64),
    MalId(i64),
}

pub async fn get(Query(param): Query<HashMap<String, i64>>) -> Result<Json<Option<PlatformLinks>>> {
    let param = serde_json::from_value::<PlatformLinksParam>(serde_json::to_value(param)?)?;

    let res = match param {
        PlatformLinksParam::AnimebytesId(id) => {
            sqlx::query_as!(
                PlatformLinksRow,
                "SELECT * FROM platform_links WHERE animebytes_id = $1 LIMIT 1",
                id
            )
            .fetch_optional(crate::DB.get().await)
            .await?
        }
        PlatformLinksParam::AnidbId(id) => {
            sqlx::query_as!(
                PlatformLinksRow,
                "SELECT * FROM platform_links WHERE anidb_id = $1 LIMIT 1",
                id
            )
            .fetch_optional(crate::DB.get().await)
            .await?
        }
        PlatformLinksParam::AnnId(id) => {
            sqlx::query_as!(
                PlatformLinksRow,
                "SELECT * FROM platform_links WHERE ann_id = $1 LIMIT 1",
                id
            )
            .fetch_optional(crate::DB.get().await)
            .await?
        }
        PlatformLinksParam::AnilistId(id) => {
            sqlx::query_as!(
                PlatformLinksRow,
                "SELECT * FROM platform_links WHERE anilist_id = $1 LIMIT 1",
                id
            )
            .fetch_optional(crate::DB.get().await)
            .await?
        }
        PlatformLinksParam::MalId(id) => {
            sqlx::query_as!(
                PlatformLinksRow,
                "SELECT * FROM platform_links WHERE mal_id = $1 LIMIT 1",
                id
            )
            .fetch_optional(crate::DB.get().await)
            .await?
        }
    };

    Ok(Json(res.map(|res| PlatformLinks {
        animebytes_id: NonZeroU64::new(res.animebytes_id as u64),
        anidb_id: NonZeroU64::new(res.anidb_id as u64),
        ann_id: NonZeroU64::new(res.ann_id as u64),
        anilist_id: NonZeroU64::new(res.anilist_id as u64),
        mal_id: NonZeroU64::new(res.mal_id as u64),
    })))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn params_deserialize() {
        let mut query = HashMap::new();
        query.insert("animebytes_id", 1);

        let params =
            serde_json::from_value::<PlatformLinksParam>(serde_json::json!(query)).unwrap();

        assert_eq!(params, PlatformLinksParam::AnimebytesId(1));
    }
}
