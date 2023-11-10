pub mod config;
pub mod domain;
pub mod routes;

use sqlx::postgres::PgPool;
use std::net::TcpListener;

pub async fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<()> {
    let app = routes::get_app(pool);
    let _ = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await;
    Ok(())
}
