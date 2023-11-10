mod health;
mod proposer;
use axum::Router;
use sqlx::postgres::PgPool;

pub fn get_app(pool: PgPool) -> Router<()> {
    let api_health = health::router();
    let api_proposer = proposer::router(pool);

    Router::new()
        .nest("/", api_health)
        .nest("/api/v1/", api_proposer)
}
