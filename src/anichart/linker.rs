use std::{collections::HashSet, time::Duration};

use anyhow::Result;
use tokio::time::sleep;

pub async fn run() {
    let mut no_results = HashSet::new();

    loop {
        if let Err(e) = inner(&mut no_results).await {
            log::error!("Failed to store platform links: {}", e);
        }

        sleep(Duration::from_secs(10)).await;
    }
}

#[allow(clippy::needless_pass_by_ref_mut)] // clippy bug??
pub async fn inner(no_results: &mut HashSet<i32>) -> Result<()> {
    let mal_id = sqlx::query!(
        "SELECT mal_id FROM platform_links
        WHERE anilist_id = 0 AND mal_id > 0"
    )
    .fetch_all(crate::DB.get().await)
    .await?
    .into_iter()
    .map(|row| row.mal_id as i32)
    .find(|mal_id| !no_results.contains(mal_id));

    if let Some(mal_id) = mal_id {
        let media = super::by_mal_id(mal_id).await;

        match media {
            Ok(media) => {
                let anilist_id = media.id;

                sqlx::query!(
                    "UPDATE platform_links
                SET anilist_id = $1
                WHERE mal_id = $2",
                    anilist_id,
                    mal_id,
                )
                .execute(crate::DB.get().await)
                .await?;
            }
            Err(e) => {
                dbg!(e);
                log::warn!("No results for MAL ID {}", mal_id);
                no_results.insert(mal_id);
            }
        }
    }

    Ok(())
}
