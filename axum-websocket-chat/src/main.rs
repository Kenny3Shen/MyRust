mod handler;
mod models;

use crate::handler::handler;
use crate::models::{AppState, MessagePayLoad};
use axum::{
    extract::ws::{Message, WebSocket},
    routing::get,
    Router,
};
use futures::{stream::SplitSink, SinkExt};
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{
    mpsc::{self},
    RwLock,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

    //server_task
    let mut server_task = tokio::spawn(async move {
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });

    tokio::select! {
            _ = (&mut sender_task) => {
                println!("Sender Exited");
                server_task.abort();
            },
            _ = (&mut server_task) => {
                println!("Sender Exited");
                sender_task.abort();
            },
            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl + C Recevied, Exited");
                sender_task.abort();
                server_task.abort();
        }
    }

    Ok(())
}

