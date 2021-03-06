use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::Path,
    sync::atomic::{AtomicI64, Ordering},
};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf},
        UnixStream,
    },
    sync::{broadcast, mpsc},
    task::JoinHandle,
};

#[derive(Clone, Debug)]
pub struct MpvAddress(pub String);

impl MpvAddress {
    pub async fn connect(&self) -> io::Result<MpvStream> {
        MpvStream::connect(&self.0).await
    }
}

pub struct MpvStream {
    tx_handle: JoinHandle<()>,
    rx_handle: JoinHandle<()>,
    command_sender: mpsc::Sender<MpvCommand>,
    // event_stream: broadcast::Sender<MpvEvent>,
    response_stream: broadcast::Sender<MpvResponse>,
    command_counter: AtomicI64,
}

impl MpvStream {
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<MpvStream> {
        let (mpv_rx, mpv_tx) = UnixStream::connect(path).await?.into_split();

        let (command_sender, command_stream) = mpsc::channel(16);

        let (event_sender, _) = broadcast::channel(16);
        // let event_stream = event_sender.clone();

        let (response_sender, _) = broadcast::channel(16);
        let response_stream = response_sender.clone();

        let tx_handle = tokio::spawn(
            MpvTxHandler {
                mpv_tx,
                command_stream,
            }
            .run(),
        );

        let rx_handle = tokio::spawn(
            MpvRxHandler {
                mpv_rx,
                event_sender,
                response_sender,
            }
            .run(),
        );

        let command_counter = AtomicI64::new(0);

        Ok(MpvStream {
            tx_handle,
            rx_handle,
            command_sender,
            // event_stream,
            response_stream,
            command_counter,
        })
    }

    pub async fn send_command(&self, command: MpvCommand) -> MpvResponse {
        let command_id = self.command_counter.fetch_add(1, Ordering::Relaxed);

        let command = command.with_id(command_id);

        let mut subscription = self.response_stream.subscribe();
        self.command_sender.send(command).await.unwrap();

        loop {
            let response = subscription.recv().await.unwrap();

            if response.request_id == Some(command_id) {
                return response;
            }
        }
    }

    // pub fn subscribe_events(&self) -> broadcast::Receiver<MpvEvent> {
    //     self.event_stream.subscribe()
    // }

    pub async fn get_property(&self, name: &str) -> Result<Option<serde_json::Value>, String> {
        let res = self
            .send_command(MpvCommand::new(["get_property", name]))
            .await;

        if let Some(error) = res.error() {
            match error {
                "property unavailable" => Ok(None),
                _ => Err(error.to_owned()),
            }
        } else if res.data.is_null() {
            Ok(None)
        } else {
            Ok(Some(res.data))
        }
    }

    pub async fn get_property_bool(&self, name: &str) -> Result<Option<bool>, String> {
        self.get_property(name)
            .await
            .map(|v| v.map(|v| v.as_bool().unwrap()))
    }

    pub async fn get_property_i32(&self, name: &str) -> Result<Option<i32>, String> {
        self.get_property(name)
            .await
            .map(|v| v.map(|v| v.as_i64().unwrap() as i32))
    }

    pub async fn get_property_f64(&self, name: &str) -> Result<Option<f64>, String> {
        self.get_property(name)
            .await
            .map(|v| v.map(|v| v.as_f64().unwrap()))
    }

    pub async fn get_property_string(&self, name: &str) -> Result<Option<String>, String> {
        self.get_property(name)
            .await
            .map(|v| v.map(|v| v.as_str().unwrap().to_owned()))
    }
}

impl Drop for MpvStream {
    fn drop(&mut self) {
        self.tx_handle.abort();
        self.rx_handle.abort();
    }
}

struct MpvTxHandler {
    mpv_tx: OwnedWriteHalf,
    command_stream: mpsc::Receiver<MpvCommand>,
}

impl MpvTxHandler {
    async fn run(mut self) {
        while let Some(cmd) = self.command_stream.recv().await {
            let mut line = serde_json::to_string(&cmd).unwrap();

            log::trace!(">> {}", line);

            line.push('\n');

            self.mpv_tx
                .write_all(line.as_bytes())
                .await
                .expect("Failed to write to mpv");
        }
    }
}

struct MpvRxHandler {
    mpv_rx: OwnedReadHalf,
    event_sender: broadcast::Sender<MpvEvent>,
    response_sender: broadcast::Sender<MpvResponse>,
}

impl MpvRxHandler {
    async fn run(self) {
        let mut reader = BufReader::new(self.mpv_rx).lines();

        while let Some(line) = reader.next_line().await.expect("mpv quit?") {
            log::trace!("<< {}", line);

            if let Ok(event) = serde_json::from_str::<MpvEvent>(&line) {
                let _ = self.event_sender.send(event);
            } else if let Ok(response) = serde_json::from_str::<MpvResponse>(&line) {
                let _ = self.response_sender.send(response);
            } else {
                log::error!("unexpected line: {}", line);
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MpvCommand {
    command: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<i64>,
}

impl MpvCommand {
    pub fn new<V, T>(command: T) -> Self
    where
        V: Into<serde_json::Value>,
        T: IntoIterator<Item = V>,
    {
        MpvCommand {
            command: command.into_iter().map(Into::into).collect(),
            request_id: None,
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.request_id = Some(id);
        self
    }
}

#[macro_export]
macro_rules! mpv_command {
    ($($arg:expr),* $(,)?) => {
        MpvCommand::new(vec![
            $(::serde_json::json!($arg)),*
        ])
    };
}

#[derive(Clone, Debug, Deserialize)]
pub struct MpvEvent {
    pub event: String,
    pub id: Option<i64>,
    pub error: Option<String>,
    #[serde(flatten)]
    pub rest: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MpvResponse {
    #[serde(default = "null")]
    pub data: serde_json::Value,
    pub error: String,
    pub request_id: Option<i64>,
}

fn null() -> serde_json::Value {
    serde_json::Value::Null
}

impl MpvResponse {
    pub fn error(&self) -> Option<&str> {
        match self.error.as_ref() {
            "success" => None,
            error => Some(error),
        }
    }
}
