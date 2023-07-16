use egui::Context;

use crate::server::interface::TetsuServerClient;

pub fn get_tetsu(ctx: &Context) -> TetsuServerClient {
    ctx.data(|d| d.get_temp::<TetsuServerClient>("tetsu".into()))
        .unwrap()
}
