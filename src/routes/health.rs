use axum::{response::IntoResponse, response::Json, routing::get, Router};
use serde_json::json;

pub fn router() -> Router<()> {
    Router::new()
        .route("/live", get(live))
        .route("/ready", get(ready))
        .route("/status", get(ready))
}

#[tracing::instrument]
async fn live() -> impl IntoResponse {
    Json(json!({ "service": "ok" }))
}

#[tracing::instrument]
async fn ready() -> impl IntoResponse {
    Json(json!({ "service": "ok" }))
}
