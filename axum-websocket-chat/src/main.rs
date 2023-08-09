use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    mpsc::{self, Sender},
    RwLock,
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<RwLock<HashMap<u16, SplitSink<WebSocket, Message>>>>,
    pub sender_tx: Sender<MessagePayLoad>,
}

#[derive(Serialize, Deserialize)]
pub struct MessagePayLoad {
    msg_type: MsgType,
    from: Option<u16>,
    to: Option<u16>,
    data: String,
}

#[derive(Serialize, Deserialize)]
pub enum MsgType {
    Message,
    Command,
    Reply,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (sender_tx, mut sender_rx) = mpsc::channel::<MessagePayLoad>(1000);

    let app_state = AppState {
        clients: Arc::new(RwLock::new(
            HashMap::<u16, SplitSink<WebSocket, Message>>::new(),
        )),
        sender_tx,
    };

    let app = Router::new()
        .route("/", get(handler))
        .with_state(app_state.clone());

    // sender_task
    let mut sender_task = tokio::spawn(async move {
        while let Some(payload) = sender_rx.recv().await {
            if let Some(to) = payload.to {
                if let Some(sender) = app_state.clients.write().await.get_mut(&to) {
                    sender
                        .send(Message::Text(json!(payload).to_string()))
                        .await
                        .unwrap();
                }
            }
        }
    });
    // sender_task

    //server_task
    let mut server_task = tokio::spawn(async move {});

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    tokio::select! {
            _ = (&mut sender_task) => {
                println!("Sender Exited");
                sender_task.abort();
            },
            _ = (&mut server_task) => {
                println!("Sender Exited");
                server_task.abort();
            },
            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl + C Recevied, Exited");
                sender_task.abort();
                server_task.abort();
        }
    }

    Ok(())
}

async fn handler(
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

/* async fn write(mut sender: SplitSink<WebSocket, Message>, mut sender_rx: Receiver<Message>) {
    while let Some(msg) = sender_rx.next().await {
        sender.send(msg).await.unwrap();
    }
} */

/* async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        // send msg error
        if socket.send(msg).await.is_err() {
            return;
        }
    }
} */
