use egui::Ui;

pub enum PageAction {
    Push(Box<dyn Page>),
    Pop,
    LoadFile(String),
}

pub trait Page {
    fn ui(&mut self, ui: &mut Ui) -> Option<PageAction>;

    fn boxx(self) -> Box<dyn Page>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
