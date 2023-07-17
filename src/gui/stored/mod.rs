use anyhow::Result;
use egui::Context;
use tarpc::context;

use self::episodes::Episodes;
use super::{
    r#async::{AsyncValue::*, AsyncValueChannel},
    utils::get_apis,
    View,
};
use crate::anidb::records::Anime;

mod episodes;

pub struct StoredView {
    anime: AsyncValueChannel<Result<Vec<Anime>>>,
    episodes: Option<Episodes>,
}

impl StoredView {
    pub fn new(ctx: &Context) -> Self {
        let tetsu = get_apis(ctx).tetsu;

        let anime =
            AsyncValueChannel::new(|_| async move { Ok(tetsu.anime(context::current()).await??) });

        Self { anime, episodes: None }
    }
}

impl View for StoredView {
    fn title(&self) -> egui::WidgetText {
        "Anime".into()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        match self.anime.get() {
            Waiting(()) => {
                ui.spinner();
            }
            Ready(Err(ref e)) => {
                ui.label(e.to_string());
            }
            Ready(Ok(ref anime)) => {
                if let Some(ref mut episodes) = self.episodes {
                    if episodes.ui(ui) {
                        self.episodes = None;
                    }
                } else {
                    for anime in anime {
                        let selected =
                            self.episodes.as_ref().map(|e| e.anime.aid) == Some(anime.aid);

                        if ui.selectable_label(selected, &anime.romaji_name).clicked() {
                            self.episodes = Some(Episodes::new(ui.ctx(), anime.clone()));
                        }
                    }
                }
            }
        }
    }
}
