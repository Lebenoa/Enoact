use std::borrow::Cow;

use axum::{
    body::Bytes,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use discord_rich_presence::activity::{
    Activity, ActivityType, Assets, StatusDisplayType, Timestamps,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::{activity_manager::ActivityManager, unwrap_or_continue_r};

pub async fn upgrade_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "SCREAMING_SNAKE_CASE")]
enum ClientMessage<'a> {
    Set {
        activity: Box<Activity<'a>>,
    },
    PresetMusic {
        title: Cow<'a, str>,
        artists: Cow<'a, str>,
        thumbnail: Option<Cow<'a, str>>,
        start: Option<i64>,
        end: Option<i64>,
    },
    Clear,
}

async fn handle_socket(socket: WebSocket) {
    let mut manager = match ActivityManager::new() {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Error spawning presence instance: {e}");
            return;
        }
    };

    let (mut sender, mut receiver) = socket.split();
    let (client_tx, mut client_rx) = mpsc::channel::<Message>(10);
    let send_task = tokio::spawn(async move {
        while let Some(msg) = client_rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let tx = client_tx.clone();

    let read_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            let Ok(msg) = msg else {
                return;
            };

            match msg {
                Message::Close(_) => {
                    break;
                }
                Message::Text(text) => {
                    let text = text.as_str();

                    let client_message: ClientMessage = unwrap_or_continue_r!(
                        serde_json::from_str(text),
                        "Failed to deserialize WS message: {}"
                    );

                    match client_message {
                        ClientMessage::Set { activity } => {
                            unwrap_or_continue_r!(
                                manager.set(*activity),
                                "Failed to set activity: {}"
                            );
                        }
                        ClientMessage::PresetMusic {
                            title,
                            artists,
                            thumbnail,
                            start,
                            end,
                        } => {
                            let large_image = thumbnail
                                .unwrap_or_else(|| "https://image.lebenoa.com/proxy/main".into());

                            let mut act = Activity::new()
                                .name("Music")
                                .activity_type(ActivityType::Listening)
                                .status_display_type(StatusDisplayType::Details)
                                .details(title)
                                .state(artists)
                                .assets(Assets::new().large_image(large_image));

                            if let Some(s) = start {
                                let mut timestamps = Timestamps::new();
                                timestamps = timestamps.start(s);
                                if let Some(e) = end {
                                    timestamps = timestamps.end(e);
                                }

                                act = act.timestamps(timestamps);
                            }

                            unwrap_or_continue_r!(
                                manager.set(act),
                                "Failed to use music preset: {}"
                            );
                        }
                        ClientMessage::Clear => {
                            unwrap_or_continue_r!(manager.clear(), "Failed to clear activity: {}");
                        }
                    };

                    if tx.send(Message::Text("Ok".into())).await.is_err() {
                        break;
                    }
                }
                _ => continue,
            }
        }
    });

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        match client_tx.send(Message::Ping(Bytes::new())).await {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }

    read_task.abort();
    send_task.abort();
}
