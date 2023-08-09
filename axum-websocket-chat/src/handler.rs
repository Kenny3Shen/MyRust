use crate::MessagePayLoad;
use std::net::SocketAddr;

use axum::{extract::{WebSocketUpgrade, State, ConnectInfo, ws::{WebSocket, Message}}, response::Response};
use futures::{StreamExt, stream::SplitStream};
use serde_json::json;

use crate::models::{AppState, MsgType};

pub async fn handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, app_state, addr))
}

async fn handle_socket(socket: WebSocket, app_state: AppState, addr: SocketAddr) {
    let (sender, receiver) = socket.split();

    app_state.clients.write().await.insert(addr.port(), sender);

    // let (sender_tx, sender_rx) = mpsc::channel::<Message>(1000);

    // tokio::spawn(write(sender, sender_rx));
    tokio::spawn(read(receiver, app_state, addr.port()));
}

async fn read(mut recevier: SplitStream<WebSocket>, app_state: AppState, client_port: u16) {
    while let Some(Ok(msg)) = recevier.next().await {
        match msg {
            Message::Text(text) => {
                println!("{text}");
                // sender_tx.send(Message::Text(text)).await.unwrap();
                let mut payload = serde_json::from_str::<MessagePayLoad>(&text).unwrap();
                match payload.msg_type {
                    MsgType::Message => {
                        payload.from = Some(client_port);
                        app_state.sender_tx.send(payload).await.unwrap();
                    }
                    MsgType::Command => {
                        if payload.data == "list" {
                            let clients = app_state
                                .clients
                                .read()
                                .await
                                .iter()
                                .map(|(k, _)| k.clone())
                                .collect::<Vec<u16>>();
                            let reply = MessagePayLoad {
                                msg_type: MsgType::Reply,
                                from: None,
                                to: Some(client_port),
                                data: json!(clients).to_string(),
                            };
                            app_state.sender_tx.send(reply).await.unwrap();
                        }
                    }
                    _ => {}
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
    app_state.clients.write().await.remove(&client_port);
}