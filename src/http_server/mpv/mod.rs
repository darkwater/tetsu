mod message;
mod process;

use std::time::Duration;

use axum::{
    extract::{
        ws::{self, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};
use futures::{
    future::{select, Either},
    pin_mut, Sink, SinkExt, Stream, StreamExt,
};
use tokio::time::sleep;

use self::{
    message::{ControlMessage, Message},
    process::MpvProcess,
};

static MPV: MpvProcess = MpvProcess::new();

pub async fn mpv_upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

fn convert(
    socket: WebSocket,
) -> impl Stream<Item = Result<Message, axum::Error>> + Sink<Message, Error = anyhow::Error> {
    socket
        .with(|msg: Message| async move { Ok(ws::Message::Text(serde_json::to_string(&msg)?)) })
        .filter_map(|msg| async move {
            if let Ok(ws::Message::Text(json)) = msg {
                let msg = serde_json::from_str::<Message>(&json).unwrap();
                Some(Ok(msg))
            } else if let Err(e) = msg {
                Some(Err(e))
            } else {
                None
            }
        })
}

async fn handle_socket(websocket: WebSocket) {
    let websocket = convert(websocket);

    pin_mut!(websocket);

    'outer: loop {
        log::debug!("starting loop");

        let running = MPV.is_running().await;

        if running {
            log::debug!("running");

            websocket
                .send(ControlMessage::Started.into())
                .await
                .unwrap();

            let mpv_socket = MPV.connect().await.unwrap();

            pin_mut!(mpv_socket);

            loop {
                match select(websocket.next(), mpv_socket.next()).await {
                    Either::Left((msg, _)) => {
                        let Some(Ok(msg)) = msg else {
                            break 'outer;
                        };

                        match msg {
                            Message::Control(ControlMessage::Stop) => {
                                MPV.stop().await;
                                continue 'outer;
                            }
                            Message::Mpv(msg) => {
                                mpv_socket.send(msg).await.unwrap();
                            }
                            Message::Control(_) => {}
                        }
                    }
                    Either::Right((msg, _)) => {
                        let Some(Ok(msg)) = msg else {
                            continue 'outer;
                        };

                        websocket.send(Message::Mpv(msg)).await.unwrap();
                    }
                }
            }
        } else {
            log::debug!("not running");

            websocket
                .send(ControlMessage::Stopped.into())
                .await
                .unwrap();

            loop {
                while let Some(msg) = websocket.next().await {
                    let msg = msg.unwrap();

                    if let Message::Control(ControlMessage::Start) = msg {
                        MPV.start().await;
                        sleep(Duration::from_secs(1)).await;
                        continue 'outer;
                    }
                }
            }
        }
    }
}
