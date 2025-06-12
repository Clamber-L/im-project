use anyhow::Result;
use axum::serve;
use server_wechat::core::entity::ApplicationEntity;
use server_wechat::init_app;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::TRACE);
    tracing_subscriber::registry().with(layer).init();

    let app_config = ApplicationEntity::try_load_yml()?;

    let addr = format!("0.0.0.0:{}", app_config.port);
    let listener = TcpListener::bind(&addr).await?;

    let router = init_app(app_config).await?;
    serve(listener, router.into_make_service()).await?;
    Ok(())
}
