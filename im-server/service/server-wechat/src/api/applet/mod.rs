pub mod entity;
pub mod service;

use crate::api::applet::service::{
    create_team, creation_list, login, operation, operation_user_num, pay, pay_callback, settings,
    update_user, user_team,
};
use crate::core::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn applet_api_router() -> Router<AppState> {
    let router = Router::new()
        .route("/login", post(login))
        .route("/user", post(update_user))
        .route("/settings", get(settings))
        .route("/creation_list", get(creation_list))
        .route("/operation", get(operation))
        .route("/team", get(user_team))
        .route("/create_team", post(create_team))
        .route("/operation_user_num", get(operation_user_num))
        .route("/pay", post(pay))
        .route("/pay_callback", post(pay_callback));
    router
}
