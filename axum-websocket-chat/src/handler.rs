use crate::models::{AppState, MessagePayLoad, MsgType};
use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{stream::SplitStream, StreamExt};
use serde_json::json;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, app_state, addr))
}

async fn handle_socket(socket: WebSocket, app_state: AppState, addr: SocketAddr) {
    let (sender, receiver) = socket.split();

    // 将接收到的消息放入发送队列
    app_state.clients.insert(addr.port(), sender);

    // let (sender_tx, sender_rx) = mpsc::channel::<Message>(1000);

    // 创建一个发送消息的线程
    // tokio::spawn(write(sender, sender_rx));
    tokio::spawn(read(receiver, app_state, addr.port()));
}

async fn read(mut receiver: SplitStream<WebSocket>, app_state: AppState, client_port: u16) {
    // 循环接收到的消息
    while let Some(Ok(msg)) = receiver.next().await {
        // 将消息转换为 json 格式
        match msg {
            Message::Text(text) => {
                println!("{text}");
                // sender_tx.send(Message::Text(text)).await.unwrap();
                let mut payload = serde_json::from_str::<MessagePayLoad>(&text).unwrap();
                match payload.msg_type {
                    MsgType::Message => {
                        // 将消息发送到指定的客户端
                        payload.from = Some(client_port);
                        app_state.sender_tx.send(payload).await.unwrap();
                    }
                    MsgType::Command => {
                        // 如果消息类型为 Command，则检查消息数据是否为 list
                        if payload.data == "list" {
                            // 获取所有客户端的 key
                            let clients = app_state
                                .clients
                                .iter()
                                .map(|x| x.key().clone())
                                .collect::<Vec<u16>>();/*  */
                            // 将 key 和消息发送到指定的客户端
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
    // 删除客户端
    app_state.clients.remove(&client_port);
}
