use anyhow::Result;
use egui::{Context, Ui};
use tarpc::context;

use crate::{
    anidb::records::{Anime, Episode},
    gui::{
        r#async::{AsyncValue::*, AsyncValueChannel},
        utils::get_tetsu,
    },
};

pub struct Episodes {
    pub anime: Anime,
    episodes: AsyncValueChannel<Result<Vec<Episode>>>,
}

impl Episodes {
    pub fn new(ctx: &Context, anime: Anime) -> Self {
        let tetsu = get_tetsu(ctx);

        let aid = anime.aid;
        let episodes = AsyncValueChannel::new(move |_| async move {
            Ok(tetsu.episodes(context::current(), aid).await??)
        });

        Self { anime, episodes }
    }

    /// returns true when we should go back
    pub fn ui(&mut self, ui: &mut Ui) -> bool {
        let back = ui.button("⬅ Back").clicked();

        ui.heading(&self.anime.romaji_name);
        ui.separator();

        match self.episodes.get() {
            Waiting(()) => {
                ui.spinner();
            }
            Ready(Err(ref e)) => {
                ui.label(e.to_string());
            }
            Ready(Ok(ref episodes)) => {
                for episode in episodes {
                    ui.label(&episode.romaji);
                }
            }
        }

        back
    }
}
