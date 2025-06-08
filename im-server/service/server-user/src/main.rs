use anyhow::Result;
use axum::serve;
use server_user::{init_app, ServerUserConfig};
use tokio::net::TcpListener;
use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

// 此服务接受一个chat-server的rpc请求，保存数据到database
// 并且推送一个新消息通知到server-message服务，由sever-message服务
#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = ServerUserConfig::try_load()?;

    let addr = format!("0.0.0.0:{}", app_config.server.port);
    let listen = TcpListener::bind(&addr).await?;

    if let Ok(app) = init_app(app_config.mysql.url.as_str(), app_config.grpc.url).await {
        serve(listen, app.into_make_service()).await?;
    } else {
        error!("Start error");
    }
    Ok(())
}
