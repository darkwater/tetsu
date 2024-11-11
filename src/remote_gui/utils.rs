use egui::Context;

use crate::server::interface::TetsuServerClient;

#[derive(Clone)]
pub struct Apis {
    pub tetsu: TetsuServerClient,
}

pub fn get_apis(ctx: &Context) -> Apis {
    ctx.data(|d| d.get_temp::<Apis>("apis".into())).unwrap()
}
