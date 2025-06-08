use std::sync::Arc;

use anyhow::Result;
use axum::http::Uri;
use axum::{routing::get, serve, Router};
use chat_server::{connect_with_websocket, send_message_with_ws, AppState, MessageGrpc};
use dashmap::DashMap;
use lib_core::{mongo_client, mysql_client, ApiResult};
use lib_rpc::pb::message::message_service_server::MessageServiceServer;
use lib_utils::chat_config::ServerChatConfig;
use lib_utils::error_result;
use tokio::net::TcpListener;
use tokio::try_join;
use tonic::transport::Server;
use tower_http::cors::{Any, CorsLayer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = ServerChatConfig::try_load()?;

    // init mongo
    let mongo_client = mongo_client(&app_config.mongo.url, &app_config.mongo.db_name).await;

    // init mysql
    let mysql_client = mysql_client(&app_config.mysql.url).await?;

    // init connection_map
    let connection_map = Arc::new(DashMap::new());

    // 初始化tonic
    let grpc_service = MessageServiceServer::new(MessageGrpc {
        connection_map: connection_map.clone(),
    });

    let app_state = AppState::new(mongo_client, mysql_client, connection_map);

    // 启动nacos
    // nacos_center(
    //     app_config.nacos.addr.as_str(),
    //     app_config.nacos.namespace.as_str(),
    //     app_config.nacos.app_name.as_str(),
    //     app_config.nacos.service_name,
    //     app_config.nacos.group,
    // )
    // .await;

    // 启动axum
    let addr = format!("0.0.0.0:{}", app_config.server.port);
    let listen = TcpListener::bind(&addr).await?;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    // init grpc server
    let grpc_addr = format!("0.0.0.0:{}", app_config.grpc.port).parse()?;
    let grpc_server = Server::builder().add_service(grpc_service).serve(grpc_addr);

    // http server
    let http_app = Router::new()
        .route("/connect/:token", get(connect_with_websocket))
        .route("/send", get(send_message_with_ws))
        .layer(cors)
        .with_state(app_state)
        .fallback(handle_404);

    let http_server = serve(listen, http_app.into_make_service());

    let grpc = async {
        grpc_server
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })
    };

    let http = async {
        http_server
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })
    };
    try_join!(grpc, http)?;
    Ok(())
}

async fn handle_404(uri: Uri) -> ApiResult<String> {
    Ok(error_result(format!("Not Found,{:?}", uri.path()).as_str()))
}
