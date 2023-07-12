use anyhow::Result;
use egui::Context;
use tarpc::context;

use super::{
    r#async::{AsyncValue::*, AsyncValueChannel},
    View,
};
use crate::{anidb::records::Anime, server::interface::TetsuServerClient};

pub struct StoredView {
    anime: AsyncValueChannel<Result<Vec<Anime>>>,
}

impl StoredView {
    pub fn new(ctx: &Context) -> Self {
        let tetsu = ctx
            .data(|d| d.get_temp::<TetsuServerClient>("tetsu".into()))
            .unwrap();

        let anime =
            AsyncValueChannel::new(|_| async move { Ok(tetsu.anime(context::current()).await??) });

        Self { anime }
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
                for anime in anime {
                    ui.label(&anime.romaji_name);
                }
            }
        }
    }
}
