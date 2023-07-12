use std::sync::mpsc;

use futures::Future;

pub struct AsyncValueChannel<T, P = ()> {
    channel: mpsc::Receiver<AsyncValue<T, P>>,
    latest: AsyncValue<T, P>,
}

pub enum AsyncValue<T, P = ()> {
    Waiting(P),
    Ready(T),
}

impl<T, P> AsyncValueChannel<T, P> {
    pub fn new<F, Fut>(fun: F) -> Self
    where
        T: Send + 'static,
        F: FnOnce(Progress<T, P>) -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send,
        P: Default + Send + 'static,
    {
        let (tx, channel) = mpsc::channel();

        let pf = Progress { tx: tx.clone() };

        tokio::task::spawn(async move {
            let v = fun(pf).await;
            let _ = tx.send(AsyncValue::Ready(v));
        });

        Self { channel, latest: Default::default() }
    }

    pub fn get(&mut self) -> &AsyncValue<T, P> {
        if let Ok(v) = self.channel.try_recv() {
            self.latest = v;
        }

        &self.latest
    }
}

impl<T, P> AsyncValue<T, P> {
    pub fn is_waiting(&self) -> bool {
        match self {
            AsyncValue::Waiting(_) => true,
            _ => false,
        }
    }

    pub fn is_ready(&self) -> bool {
        match self {
            AsyncValue::Ready(_) => true,
            _ => false,
        }
    }
}

impl<T, P: Default> Default for AsyncValue<T, P> {
    fn default() -> Self {
        Self::Waiting(Default::default())
    }
}

pub struct Progress<T, P> {
    tx: mpsc::Sender<AsyncValue<T, P>>,
}

impl<T, P> Progress<T, P> {
    pub fn report(&self, progress: P) {
        let _ = self.tx.send(AsyncValue::Waiting(progress));
    }
}
