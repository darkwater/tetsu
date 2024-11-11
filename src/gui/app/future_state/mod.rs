use std::{hash::Hash, sync::Arc};

use anyhow::Result;
use egui::{mutex::Mutex, Color32, Context, Id, Ui, UiBuilder, Widget};
use tokio::sync::oneshot::{error::TryRecvError, Receiver};

// TODO: method to remove state from "removed" widgets

type ReadyBuilder<'a, T> = Box<dyn FnOnce(&mut Ui, &mut <T as FutureState>::State) + 'a>;
type LoadingBuilder<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;
type ErrorBuilder<'a> = Box<dyn FnOnce(&mut Ui, &anyhow::Error) + 'a>;

pub struct FutureUi<'a, T: FutureState> {
    input: T,
    ready_builder: ReadyBuilder<'a, T>,
    loading_builder: Option<LoadingBuilder<'a>>,
    error_builder: Option<ErrorBuilder<'a>>,
}

impl<T: FutureState> FutureUi<'_, T> {
    fn loading_ui(self, builder: impl FnOnce(&mut Ui) + 'static) -> Self {
        Self {
            loading_builder: Some(Box::new(builder)),
            ..self
        }
    }

    fn error_ui(self, builder: impl FnOnce(&mut Ui, &anyhow::Error) + 'static) -> Self {
        Self {
            error_builder: Some(Box::new(builder)),
            ..self
        }
    }
}

fn maybe_error_ui(builder: Option<ErrorBuilder>, ui: &mut Ui, e: &anyhow::Error) {
    if let Some(f) = builder {
        f(ui, e);
    } else {
        ui.colored_label(Color32::RED, format!("Error: {}", e));
    }
}

fn maybe_loading_ui(builder: Option<LoadingBuilder>, ui: &mut Ui) {
    if let Some(f) = builder {
        f(ui);
    } else {
        ui.spinner();
    }
}

impl<T: FutureState> Widget for FutureUi<'_, T> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.allocate_new_ui(UiBuilder::new(), |ui| {
            let id = Id::new(&self.input);
            let state = ui.data(|data| data.get_temp::<Arc<FutureStateData<T::State>>>(id));

            match state.as_deref() {
                Some(FutureStateData::Ok(state)) => {
                    (self.ready_builder)(ui, &mut state.lock());
                }
                Some(FutureStateData::Err(e)) => {
                    maybe_error_ui(self.error_builder, ui, e);
                }
                Some(FutureStateData::Waiting(rx)) => match rx.lock().try_recv() {
                    Ok(mut res) => {
                        match &mut res {
                            Ok(ref mut state) => {
                                (self.ready_builder)(ui, state);
                            }
                            Err(e) => {
                                maybe_error_ui(self.error_builder, ui, e);
                            }
                        }

                        ui.data_mut(|data| {
                            data.insert_temp::<Arc<FutureStateData<T::State>>>(
                                id,
                                Arc::new(res.into()),
                            );
                        });
                    }
                    Err(TryRecvError::Empty) => maybe_loading_ui(self.loading_builder, ui),
                    Err(TryRecvError::Closed) => {
                        unimplemented!("FutureUi receiver closed");
                    }
                },
                None => {
                    let (tx, rx) = tokio::sync::oneshot::channel::<Result<T::State>>();
                    let this = self.input.clone();
                    let ctx = ui.ctx().clone();
                    tokio::spawn(async move {
                        let res: Result<T::State> = this.load(ctx).await;
                        let _ = tx.send(res);
                    });

                    ui.data_mut(|data| {
                        data.insert_temp::<Arc<FutureStateData<T::State>>>(
                            id,
                            Arc::new(FutureStateData::Waiting(Mutex::new(rx))),
                        );
                    });

                    maybe_loading_ui(self.loading_builder, ui);
                }
            }
        })
        .response
    }
}

pub trait FutureState: Clone + Send + Hash + 'static {
    type State: Send + 'static;

    fn load(
        self,
        ctx: Context,
    ) -> impl std::future::Future<Output = Result<Self::State>> + std::marker::Send;

    fn ready_ui<'a>(
        &self,
        builder: impl FnOnce(&mut Ui, &mut Self::State) + 'a,
    ) -> FutureUi<'a, Self> {
        FutureUi {
            input: self.clone(),
            ready_builder: Box::new(builder),
            loading_builder: None,
            error_builder: None,
        }
    }
}

enum FutureStateData<T> {
    Waiting(Mutex<Receiver<Result<T>>>),
    Ok(Mutex<T>),
    Err(anyhow::Error),
}

impl<T> From<Result<T>> for FutureStateData<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(v) => FutureStateData::Ok(Mutex::new(v)),
            Err(e) => FutureStateData::Err(e),
        }
    }
}
