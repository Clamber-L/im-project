use axum::http::{Method, StatusCode, Uri};
use axum::routing::get;
use axum::{serve, BoxError, Router, ServiceExt};
use std::time::Duration;
use tokio::net::TcpListener;
use tower::timeout::TimeoutLayer;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!".to_owned() }))
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .handle_error(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        (StatusCode::REQUEST_TIMEOUT, "Request timed out".to_string())
                    } else {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled error: {}", error),
                        )
                    }
                }),
        );
    let addr = "0.0.0.0:9999".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();

    println!("Hello, world!");
}

async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{method} {uri}` failed with {err}"),
    )
}

async fn handle_anyhow_error(
    error: Box<dyn std::error::Error + Send + Sync>,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {error}"),
    )
}
