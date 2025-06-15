use crate::api::applet::applet_api_router;
use crate::api::auth::wechat_api_router;
use crate::api::common::common_api_router;
use crate::core::entity::ApplicationEntity;
use crate::core::AppState;
use anyhow::Result;
use axum::routing::get;
use axum::Router;
use lib_core::{mysql_client, redis_client, RedisService};
use tower_http::cors::{Any, CorsLayer};

mod api;
pub mod core;

pub async fn init_app(application: ApplicationEntity) -> Result<Router> {
    let redis_client = redis_client(application.redis.url.as_str()).await?;
    let mysql_client = mysql_client(application.mysql.url.as_str()).await?;

    let app_state = AppState::new(
        application.wechat,
        application.applet,
        RedisService::new(redis_client),
        mysql_client,
        application.oss,
        application.pay,
    );

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/common", common_api_router())
        .nest("/wechat", wechat_api_router())
        .nest("/applet", applet_api_router())
        .layer(cors)
        .with_state(app_state);
    Ok(router)
}
