use anyhow::Result;
use axum::serve;
use lib_utils::auth_config::ServerAuthConfig;
use server_auth::app_router;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

/// 登录鉴权服务
#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = ServerAuthConfig::try_load()?;

    let addr = format!("0.0.0.0:{}", app_config.server.port);
    let listen = TcpListener::bind(&addr).await?;

    if let Ok(app) = app_router(app_config.mysql.url.as_str()).await {
        serve(listen, app.into_make_service()).await?;
    }
    Ok(())
}
