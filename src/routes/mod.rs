mod health;
use axum::Router;

pub fn get_app() -> Router {
    let api_health = health::get_router();

    Router::new().nest("/", api_health)
}
