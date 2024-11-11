use egui::{Context, Response};

pub trait AutofocusExt {
    fn autofocus(self, ctx: &Context) -> Self;
}

impl AutofocusExt for Response {
    fn autofocus(self, ctx: &Context) -> Self {
        if ctx.memory(|m| m.focused().is_none()) {
            ctx.memory_mut(|m| m.request_focus(self.id));
        }
        self
    }
}
