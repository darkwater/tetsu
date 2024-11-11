use egui::Context;

use self::status_panel::StatusPanel;
use super::View;

mod status_panel;

pub struct AnimebytesView {
    status_panel: StatusPanel,
}

impl AnimebytesView {
    pub fn new(ctx: &Context) -> Self {
        let status_panel = StatusPanel::new(ctx);

        Self { status_panel }
    }
}

impl View for AnimebytesView {
    fn title(&self) -> egui::WidgetText {
        "AnimeBytes".into()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::bottom("animebytes stats")
            .height_range(ui.spacing().interact_size.y * 2.0..=ui.spacing().interact_size.y * 3.)
            .show_inside(ui, |ui| self.status_panel.ui(ui));

        egui::CentralPanel::default().show_inside(ui, |_ui| {});
    }
}
