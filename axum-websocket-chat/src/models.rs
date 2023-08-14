use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
#[derive(Clone)]
pub struct AppState {
    // pub clients: Arc<RwLock<HashMap<u16, SplitSink<WebSocket, Message>>>>,
    // DashMap 可以直接替代 RwLock<HashMap<K, V, S>>
    // DashMap 的 locking behavior acts 使用 RwLock 保证线程安全 （insert, remove, iter...）
    pub clients: Arc<DashMap<u16, SplitSink<WebSocket, Message>>>,
    pub sender_tx: Sender<MessagePayLoad>,
}

#[derive(Serialize, Deserialize)]
pub struct MessagePayLoad {
    pub msg_type: MsgType,
    pub from: Option<u16>,
    pub to: Option<u16>,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub enum MsgType {
    Message,
    Command,
    Reply,
}
