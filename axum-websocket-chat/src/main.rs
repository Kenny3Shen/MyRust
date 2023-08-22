mod handler;
mod models;

use crate::handler::handler;
use crate::models::{AppState, MessagePayLoad};
use axum::{
    extract::ws::{Message, WebSocket},
    routing::get,
    Router,
};
use dashmap::DashMap;
use futures::{stream::SplitSink, SinkExt};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc::{self};

// 定义 Result 类型
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个发送消息的通道
    let (sender_tx, mut sender_rx) = mpsc::channel::<MessagePayLoad>(1000);

    // 创建一个应用状态
    let app_state = AppState {
        // clients: Arc::new(RwLock::new(
        //     HashMap::<u16, SplitSink<WebSocket, Message>>::new(),
        // )),
        // 创建一个 Arc 实例，用于保存客户端连接
        clients: Arc::new(DashMap::<u16, SplitSink<WebSocket, Message>>::new()),
        sender_tx,
    };

    // 创建一个路由
    let app = Router::new()
        .route("/", get(handler))
        .with_state(app_state.clone());

    // sender_task
    // 创建一个发送任务
    let mut sender_task = tokio::spawn(async move {
        // 遍历发送消息的消息
        while let Some(payload) = sender_rx.recv().await {
            // 如果消息的接收者存在
            if let Some(to) = payload.to {
                // 如果消息的发送者存在
                if let Some(mut sender) = app_state.clients.get_mut(&to) {
                    // 发送消息
                    sender
                        .value_mut()
                        .send(Message::Text(json!(payload).to_string()))
                        .await
                        .unwrap();
                }
            }
        }
    });

    //server_task
    // 创建一个服务器任务
    let mut server_task = tokio::spawn(async move {
        // 绑定端口 3000
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    });

    tokio::select! {
            _ = (&mut sender_task) => {
                // 打印 Sender Exited
                println!("Sender Exited");
                // 取消发送任务
                server_task.abort();
            },
            _ = (&mut server_task) => {
                // 打印 Sender Exited
                println!("Sender Exited");
                // 取消发送任务
                sender_task.abort();
            },
            _ = tokio::signal::ctrl_c() => {
                // 打印 Ctrl + C Received, Exited
                println!("Ctrl + C Received, Exited");
                // 取消发送任务
                sender_task.abort();
                // 取消服务器任务
                server_task.abort();
        }
    }

    // 返回结果
    Ok(())
}
