pub mod entity;
pub mod service;

use crate::api::applet::service::{creation_list, login, operation, pay, update_user, user_team};
use crate::core::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn applet_api_router() -> Router<AppState> {
    let router = Router::new()
        .route("/login", post(login))
        .route("/user", post(update_user))
        .route("/creation_list", get(creation_list))
        .route("/operation", get(operation))
        .route("/team", get(user_team))
        .route("/pay", post(pay));
    router
}
