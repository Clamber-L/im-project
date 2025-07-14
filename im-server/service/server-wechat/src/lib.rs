use crate::api::applet::applet_api_router;
use crate::api::auth::wechat_api_router;
use crate::api::common::common_api_router;
use crate::core::entity::ApplicationEntity;
use crate::core::AppState;
use aliyun_oss_rust_sdk::oss::OSS;
use anyhow::Result;
use axum::routing::get;
use axum::Router;
use lib_core::{mysql_client, redis_client, AppError, RedisService};
use std::fs::read_to_string;
use tower_http::cors::{Any, CorsLayer};
use wechat_pay_rust_sdk::pay::WechatPay;

mod api;
pub mod core;

pub async fn init_app(application: ApplicationEntity) -> Result<Router, AppError> {
    let redis_client = redis_client(application.redis.url.as_str()).await?;
    let mysql_client = mysql_client(application.mysql.url.as_str()).await?;

    let pay_config = application.pay;
    // 初始化支付
    let path = pay_config.key_path;
    let contents = read_to_string(path);
    if contents.is_err() {
        return Err(AppError::ServiceError("初始化微信支付失败"));
    }

    let contents = contents.unwrap();
    let wechat_pay = WechatPay::new(
        &pay_config.app_id,
        &pay_config.mch_id,
        &contents,
        &pay_config.serial_no,
        &pay_config.v3_key,
        &pay_config.notify_url,
    );

    let oss_config = application.oss;
    let oss = OSS::new(
        &oss_config.key,
        &oss_config.secret,
        &oss_config.end_point,
        &oss_config.bucket,
    );

    let app_state = AppState::new(
        application.wechat,
        application.applet,
        RedisService::new(redis_client),
        mysql_client,
        oss,
        wechat_pay,
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
