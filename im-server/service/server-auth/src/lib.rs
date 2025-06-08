mod common;
mod state;

use crate::auth::{insert_user, login, signup};
use anyhow::Result;
use axum::http::Method;
use axum::routing::{get, post};
use axum::{middleware, Router};
pub use common::*;
use lib_core::mysql_client;
use lib_core::verification_token::verification_header;
pub use state::AppState;
use tower_http::cors::{Any, CorsLayer};

pub async fn app_router(mysql_url: &str) -> Result<Router> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let mysql_client = mysql_client(mysql_url).await?;
    let app_state = AppState::new(mysql_client);

    // need token verification
    let secure_routes = Router::new()
        .route("/add", get(insert_user))
        .layer(middleware::from_fn(verification_header))
        .with_state(app_state.clone());

    // not require token router
    let no_token_router = Router::new()
        .route("/login", get(login))
        .route("/signup", post(signup))
        .with_state(app_state.clone());

    let app = Router::new()
        .merge(secure_routes)
        .merge(no_token_router)
        .layer(cors);

    Ok(app)
}
