use axum::extract::ws::{WebSocket, Message};
use futures::stream::SplitSink;
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc::Sender};
use std::{collections::HashMap, sync::Arc};
#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<RwLock<HashMap<u16, SplitSink<WebSocket, Message>>>>,
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