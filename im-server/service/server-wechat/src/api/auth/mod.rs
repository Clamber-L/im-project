use crate::api::auth::service::sign_token;
use crate::core::AppState;
use axum::routing::get;
use axum::Router;

mod entity;
pub mod service;

pub fn wechat_api_router() -> Router<AppState> {
    let router = Router::new().route("/token", get(sign_token));
    router
}
