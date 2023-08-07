use std::{collections::HashMap, result::Result as StdResult};

use anyhow::Context;
use axum::extract::Query;
use serde::Deserialize;

use crate::{db::settings, http_server::Result};

pub async fn search(Query(mut params): Query<HashMap<String, String>>) -> Result<String> {
    let username = settings::animebytes::username()
        .await?
        .context("No username set")?;

    let torrentkey = settings::animebytes::torrentkey()
        .await?
        .context("No torrentkey set")?;

    params.insert("username".to_string(), username);
    params.insert("torrent_pass".to_string(), torrentkey);

    let url = "https://animebytes.tv/scrape.php";
    let res = reqwest::Client::new()
        .get(url)
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    let res2 = res.clone();

    tokio::spawn(async move {
        if let Err(e) = store_data(&res2).await {
            log::error!("Failed to cache Animebytes data: {}", e);
        }
    });

    Ok(res)
}

async fn store_data(body: &str) -> anyhow::Result<()> {
    let parsed: ScrapeResponse = serde_json::from_str(body)?;

    for group in parsed.groups {
        let json = serde_json::to_string(&group)?;
        let group: ScrapeGroup = serde_json::from_value(group)?;

        sqlx::query!(
            "INSERT INTO animebytes_groups (id, data)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET data = $2",
            group.id,
            json,
        )
        .execute(crate::DB.get().await)
        .await?;

        for torrent in group.torrents {
            sqlx::query!(
                "INSERT INTO animebytes_torrents (torrent_id, group_id)
                VALUES ($1, $2)
                ON CONFLICT (torrent_id) DO NOTHING",
                group.id,
                torrent.id,
            )
            .execute(crate::DB.get().await)
            .await?;
        }

        // store platform links

        let split_on = |delim| {
            move |input: Option<String>| {
                input.and_then(|url| {
                    url.split(delim)
                        .find(|c| c.chars().any(|c| c.is_numeric()))
                        .and_then(|c| c.parse::<i32>().ok())
                })
            }
        };

        let anidb = split_on('/')(group.links.anidb);
        let ann = split_on('=')(group.links.ann);
        let mal = split_on('/')(group.links.mal);

        let anidb_or_zero = anidb.unwrap_or(0);
        let ann_or_zero = ann.unwrap_or(0);
        let mal_or_zero = mal.unwrap_or(0);

        let link = sqlx::query!(
            "SELECT * FROM platform_links
            WHERE animebytes_id = $1
            OR (anidb_id = $2 AND anidb_id > 0)
            OR (ann_id = $3 AND ann_id > 0)
            OR (mal_id = $4 AND mal_id > 0)",
            group.id,
            anidb_or_zero,
            ann_or_zero,
            mal_or_zero,
        )
        .fetch_optional(crate::DB.get().await)
        .await?;

        if let Some(link) = link {
            let anidb = anidb.unwrap_or(link.anidb_id as i32);
            let ann = ann.unwrap_or(link.ann_id as i32);
            let mal = mal.unwrap_or(link.mal_id as i32);

            sqlx::query!(
                "UPDATE platform_links
                SET animebytes_id = $1, anidb_id = $2, ann_id = $3, mal_id = $4
                WHERE id = $5",
                group.id,
                anidb,
                ann,
                mal,
                link.id,
            )
            .execute(crate::DB.get().await)
            .await?;
        } else {
            sqlx::query!(
                "INSERT INTO platform_links (animebytes_id, anidb_id, ann_id, mal_id)
                VALUES ($1, $2, $3, $4)",
                group.id,
                anidb_or_zero,
                ann_or_zero,
                mal_or_zero,
            )
            .execute(crate::DB.get().await)
            .await?;
        }
    }

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ScrapeResponse {
    groups: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ScrapeGroup {
    #[serde(rename = "ID")]
    id: u32,
    // either empty array or Map<String, String>
    #[serde(deserialize_with = "maybe_deserialize_map")]
    links: ScrapeLinks,
    torrents: Vec<ScrapeTorrent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ScrapeLinks {
    #[serde(rename = "AniDB")]
    anidb: Option<String>,
    #[serde(rename = "ANN")]
    ann: Option<String>,
    #[serde(rename = "MAL")]
    mal: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScrapeTorrent {
    /// ID of torrent
    #[serde(rename = "ID")]
    id: u32,
}

fn maybe_deserialize_map<'de, D>(deserializer: D) -> StdResult<ScrapeLinks, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    if value.is_object() {
        Ok(serde_json::from_value(value).unwrap())
    } else {
        Ok(ScrapeLinks { anidb: None, ann: None, mal: None })
    }
}
