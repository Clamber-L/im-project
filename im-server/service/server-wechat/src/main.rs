use anyhow::Result;
use axum::serve;
use lib_core::init_logger;
use server_wechat::core::entity::ApplicationEntity;
use server_wechat::init_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // 读取配置
    let app_config = ApplicationEntity::try_load_yml()?;

    // 初始化日志模块
    let _guard = init_logger(app_config.project.name.as_str());

    let addr = format!("0.0.0.0:{}", app_config.project.port);
    let listener = TcpListener::bind(&addr).await?;

    let router = init_app(app_config).await?;
    serve(listener, router.into_make_service()).await?;
    Ok(())
}
