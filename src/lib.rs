pub mod domain;
pub mod routes;

use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> hyper::Result<()> {
    let app = routes::get_app();
    let _ = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await;
    Ok(())
}
