pub mod handler;
pub mod state;

use crate::state::AppState;
use crate::user::{add_friend, chat_list};
use anyhow::Result;
use axum::http::Method;
use axum::routing::{get, post};
use axum::{middleware, Router};
pub use handler::*;
use lib_core::mysql_client;
use lib_core::verification_token::verification_header;
use lib_rpc::pb::message::message_service_client::MessageServiceClient;
pub use lib_utils::user_config::*;
use tower_http::cors::{Any, CorsLayer};

pub async fn init_app(mysql_url: &str, grpc_url: String) -> Result<Router> {
    let mysql_client = mysql_client(mysql_url).await?;

    // 初始化一个grpc客户端
    let grpc_client = MessageServiceClient::connect(grpc_url).await?;

    let app_state = AppState::new(mysql_client, grpc_client);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/add_friend", post(add_friend))
        .route("/chat_list", get(chat_list))
        .layer(middleware::from_fn(verification_header))
        .layer(cors)
        .with_state(app_state);
    Ok(router)
}
