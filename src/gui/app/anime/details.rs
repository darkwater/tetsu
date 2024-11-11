use anyhow::{Context as _, Result};
use egui::{Context, Ui};
use futures::{StreamExt as _, TryStreamExt as _};

use crate::{
    anidb::records::{Anime, Episode, File, Group},
    gui::app::{
        future_state::FutureState,
        page::{Page, PageAction},
    },
};

#[derive(Clone, Hash)]
struct Episodes {
    aid: u32,
}

struct EpisodeListing {
    episode: Episode,
    files: Vec<FileListing>,
}

struct FileListing {
    file: File,
    group: Group,
    paths_on_disk: Vec<String>,
}

impl FutureState for Episodes {
    type State = Vec<EpisodeListing>;

    async fn load(self, _ctx: Context) -> Result<Self::State> {
        let db = crate::DB.get().await;

        let mut episodes = sqlx::query!("SELECT json FROM episodes WHERE aid = ?", self.aid)
            .fetch_all(db)
            .await?
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

        let mut listings = vec![];

        for episode in episodes {
            let queries = sqlx::query!(
                "SELECT f.json as fjson, g.json as gjson FROM files f
                INNER JOIN groups g ON f.gid = g.gid
                WHERE f.eid = ?",
                episode.eid
            )
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| async move {
                let file: File =
                    serde_json::from_str(&row.fjson).context("Invalid record in database")?;
                let group: Group =
                    serde_json::from_str(&row.gjson).context("Invalid record in database")?;

                let paths_on_disk =
                    sqlx::query_scalar!("SELECT path FROM indexed_files WHERE fid = ?", file.fid)
                        .fetch_all(db)
                        .await?;

                Result::<_, anyhow::Error>::Ok(FileListing { file, group, paths_on_disk })
            });

            let files = tokio_stream::iter(queries)
                .buffer_unordered(10)
                .try_collect::<Vec<FileListing>>()
                .await?;

            if !files.is_empty() {
                listings.push(EpisodeListing { episode, files });
            }
        }

        Ok(listings)
    }
}

pub struct AnimeDetails(pub Anime);

impl Page for AnimeDetails {
    fn ui(&mut self, ui: &mut Ui) -> Option<PageAction> {
        let mut action = None;

        if ui.button("<<").clicked() {
            action = Some(PageAction::Pop);
        }

        ui.add(Episodes { aid: self.0.aid }.ready_ui(|ui, state| {
            for item in state {
                if ui.button(&item.episode.romaji).clicked() {
                    action = Some(PageAction::LoadFile(
                        item.files
                            .first()
                            .unwrap()
                            .paths_on_disk
                            .first()
                            .unwrap()
                            .clone(),
                    ));
                }
            }
        }));

        action
    }
}
