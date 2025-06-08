mod proto_service;
mod state;

use dashmap::DashMap;
use lib_core::{verification_jwt, ApiResult, ExtractQuery, JwtUser};
use lib_utils::ok_result_with_none;
pub use proto_service::*;
use serde::{Deserialize, Serialize};
pub use state::*;
use std::{ops::DerefMut, sync::Arc};

use axum::extract::Path;
use axum::extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

type Sender = SplitSink<WebSocket, Message>;
pub type ConnectionMap = Arc<DashMap<String, Sender>>;

pub async fn connect_with_websocket(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> ApiResult<String> {
    println!("param:{:?}", token);
    let token_result = verification_jwt(&token);
    if let Ok(user) = token_result {
        let _ = ws.on_upgrade(|socket| handle_connection(socket, state, user));
    }
    Ok(ok_result_with_none())
}

async fn handle_connection(socket: WebSocket, state: AppState, user: JwtUser) {
    println!("user: {:?}", user);
    let (sender, receiver) = socket.split();

    state.connection_map.insert(user.id, sender);

    tokio::spawn(receive_message(receiver, state));
}

async fn receive_message(mut reveiver: SplitStream<WebSocket>, _state: AppState) {
    while let Some(msg) = reveiver.next().await {
        // 收到消息后 拉取timeline
        if let Ok(message) = msg {
            match message {
                Message::Text(msg) => {
                    // 发送消息到chat-logic服务，需要保证消息的幂等性
                    println!("receive msg: {:?}", msg);
                }
                Message::Binary(_vec) => todo!(),
                Message::Ping(_) => {
                    println!("receive ping message")
                }
                Message::Pong(_vec) => todo!(),
                Message::Close(_) => {
                    println!("client close connection")
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SendMessageParam {
    id: String,
}

pub async fn send_message_with_ws(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<SendMessageParam>,
) -> ApiResult<String> {
    if let Some(mut connection_id) = state.connection_map.clone().get_mut(&param.id) {
        connection_id
            .deref_mut()
            .send(Message::Text("message".to_string().into()))
            .await?;
    }

    Ok(ok_result_with_none())
}
