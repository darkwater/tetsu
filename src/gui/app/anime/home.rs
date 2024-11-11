use anyhow::{Context as _, Result};
use egui::{Context, Ui};

use crate::{
    anidb::records::Anime,
    gui::app::{
        future_state::FutureState,
        page::{Page, PageAction},
    },
};

use super::details::AnimeDetails;

#[derive(Clone, Hash)]
struct Shows;

impl FutureState for Shows {
    type State = Vec<Anime>;

    async fn load(self, _ctx: Context) -> Result<Self::State> {
        let db = crate::DB.get().await;

        let mut anime = sqlx::query!("SELECT json FROM anime")
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
            .collect::<Result<Vec<Anime>>>()?;

        anime.sort_by(|a, b| a.romaji_name.cmp(&b.romaji_name));

        Ok(anime)
    }
}

pub struct AnimeHome;

impl Page for AnimeHome {
    fn ui(&mut self, ui: &mut Ui) -> Option<PageAction> {
        let mut action = None;
        ui.add(Shows.ready_ui(|ui, state| {
            for anime in state {
                if ui.button(&anime.romaji_name).clicked() {
                    action = Some(PageAction::Push(Box::new(AnimeDetails(anime.clone()))));
                }
            }
        }));
        action
    }
}
