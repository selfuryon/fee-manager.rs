mod health;
mod proposer;
use axum::Router;

pub fn get_app() -> Router {
    let api_health = health::get_router();
    let api_proposer = proposer::get_router();

    Router::new()
        .nest("/", api_health)
        .nest("/api/v1/", api_proposer)
}
