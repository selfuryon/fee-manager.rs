use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn get_router() -> Router {
    Router::new()
        .route("/live", get(live))
        .route("/ready", get(ready))
}

#[tracing::instrument]
async fn live() -> impl IntoResponse {
    (StatusCode::OK, "Service is alive")
}

#[tracing::instrument]
async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "Service is ready")
}
