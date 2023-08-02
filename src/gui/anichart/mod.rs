use anyhow::Result;
use egui::Context;

use super::{
    r#async::{AsyncValue::*, AsyncValueChannel},
    View,
};
use crate::anichart::{self, data::airing::Media};

pub struct AnichartView {
    list: AsyncValueChannel<Result<Vec<Media>>>,
}

impl AnichartView {
    pub fn new(_ctx: &Context) -> Self {
        let list = AsyncValueChannel::new(|_| async move { anichart::airing().await });

        Self { list }
    }
}

impl View for AnichartView {
    fn title(&self) -> egui::WidgetText {
        "AniChart".into()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        match self.list.get() {
            Waiting(()) => {
                ui.spinner();
            }
            Ready(Err(ref e)) => {
                ui.label(e.to_string());
            }
            Ready(Ok(ref page)) => {
                for anime in page {
                    ui.label(&anime.title.romaji);
                }
            }
        }
    }
}
