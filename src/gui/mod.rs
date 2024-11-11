use anyhow::Result;
use eframe::egui;
use egui_dock::{DockState, NodeIndex, Style};

use self::{anichart::AnichartView, animebytes::AnimebytesView, stored::StoredView, utils::Apis};
use crate::server::interface::TetsuServerClient;

mod anichart;
mod animebytes;
mod r#async;
mod stored;
mod utils;

pub async fn run() -> Result<()> {
    log::debug!("Connecting to server");
    let tetsu = crate::server::connect("127.0.0.1:5352").await?;
    log::debug!("Connected to server");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tetsu",
        native_options,
        Box::new(|cc| Ok(Box::new(TetsuGuiApp::new(tetsu, cc)))),
    )
    .unwrap();

    Ok(())
}

struct TetsuGuiApp {
    state: DockState<Box<dyn View>>,
}

impl TetsuGuiApp {
    fn new(tetsu: TetsuServerClient, cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx
            .data_mut(|d| d.insert_temp("apis".into(), Apis { tetsu }));

        let left: Vec<Box<dyn View>> = vec![Box::new(StoredView::new(&cc.egui_ctx))];
        let mut state = DockState::new(left);

        let right: Vec<Box<dyn View>> = vec![
            Box::new(AnichartView::new(&cc.egui_ctx)),
            Box::new(AnimebytesView::new(&cc.egui_ctx)),
        ];
        state
            .main_surface_mut()
            .split_right(NodeIndex::root(), 0.5, right);

        Self { state }
    }
}

impl eframe::App for TetsuGuiApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui_dock::DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show_close_buttons(false)
            .show(ctx, &mut TabViewer {});
    }
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = Box<dyn View>;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title()
    }
}

trait View {
    fn title(&self) -> egui::WidgetText;
    fn ui(&mut self, ui: &mut egui::Ui);
}
