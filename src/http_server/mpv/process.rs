use futures::{Sink, SinkExt, Stream, StreamExt};
use tokio::{
    net::UnixStream,
    process::{Child, Command},
    sync::RwLock,
};
use tokio_util::codec::{Framed, LinesCodec};

pub struct MpvProcess {
    process: RwLock<Option<Child>>,
}

impl MpvProcess {
    pub const fn new() -> Self {
        Self { process: RwLock::const_new(None) }
    }

    pub async fn is_running(&self) -> bool {
        self.process.read().await.is_some()
    }

    pub async fn start(&self) {
        if self.is_running().await {
            return;
        }

        let process = Command::new("mpv")
            .arg("--idle")
            .arg("--force-window")
            .arg("--input-ipc-server=/tmp/mpvsocket")
            .spawn()
            .unwrap();

        self.process.write().await.replace(process);
    }

    pub async fn stop(&self) {
        if !self.is_running().await {
            return;
        }

        let mut process = self.process.write().await;

        // TODO: ask nicely first

        if let Some(mut process) = process.take() {
            process.kill().await.unwrap();
        }
    }

    pub async fn connect(
        &self,
    ) -> anyhow::Result<
        impl Stream<Item = anyhow::Result<serde_json::Value>>
            + Sink<serde_json::Value, Error = anyhow::Error>,
    > {
        let socket = UnixStream::connect("/tmp/mpvsocket").await?;
        let framed = Framed::new(socket, LinesCodec::new());

        Ok(framed
            .map(|msg| {
                let msg = msg.unwrap();
                let msg = serde_json::from_str(&msg).unwrap();
                Ok(msg)
            })
            .with(|msg| async move { Ok(serde_json::to_string(&msg)?) }))
    }
}
