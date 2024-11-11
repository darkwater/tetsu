use anyhow::Result;
use egui::{Color32, Context, NumExt, Response, Sense, Stroke, TextStyle, Ui, Vec2, WidgetText};

use crate::{
    animebytes::data::status::{Status, StatusResponse},
    remote_gui::r#async::{AsyncValue::*, AsyncValueChannel},
};

pub struct StatusPanel {
    status: AsyncValueChannel<Result<StatusResponse>>,
}

impl StatusPanel {
    pub fn new(_ctx: &Context) -> Self {
        let status = AsyncValueChannel::new(|_| async move { crate::animebytes::status().await });

        Self { status }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self.status.get() {
            Waiting(()) => {
                ui.spinner();
            }
            Ready(Err(ref e)) => {
                ui.label(e.to_string());
            }
            Ready(Ok(ref status)) => {
                ui.horizontal(|ui| {
                    status_icon(ui, status.tracker.status, "Trackers");
                    status_icon(ui, status.site, "Site");
                });
                ui.horizontal(|ui| {
                    for tracker in &status.tracker.details {
                        status_icon(ui, tracker.status, "").on_hover_text(&tracker.ip);
                    }
                });
            }
        }
    }
}

fn status_icon(ui: &mut Ui, status: Status, label: &str) -> Response {
    let spacing = &ui.spacing();
    let icon_width = spacing.icon_width;
    let icon_spacing = spacing.icon_spacing;

    let total_extra = egui::vec2(icon_width + icon_spacing, 0.0);

    let wrap_width = ui.available_width() - total_extra.x;
    let text = WidgetText::from(label).into_galley(ui, None, wrap_width, TextStyle::Button);

    let mut desired_size = total_extra + text.size();
    desired_size = desired_size.at_least(Vec2::splat(icon_width));
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().noninteractive();
        let (small_icon_rect, _big) = ui.spacing().icon_rectangles(rect);

        let (fill, stroke) = match status {
            Status::Offline => (Color32::RED, Color32::DARK_RED),
            Status::Online => (Color32::GREEN, Color32::DARK_GREEN),
            Status::MaintenanceOrPartialOutage => (Color32::YELLOW, Color32::BROWN),
        };

        ui.painter().circle(
            small_icon_rect.center(),
            small_icon_rect.width() / 2.,
            fill,
            Stroke { color: stroke, width: 1. },
        );

        let text_pos = egui::pos2(
            rect.min.x + icon_width + icon_spacing,
            rect.center().y - 0.5 * text.size().y,
        );

        ui.painter().galley(text_pos, text, visuals.fg_stroke.color);
    }

    response
}
