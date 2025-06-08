use crate::api::common::service::upload;
use crate::core::AppState;
use axum::routing::post;
use axum::Router;

mod service;

pub fn common_api_router() -> Router<AppState> {
    Router::new().route("/upload", post(upload))
}
