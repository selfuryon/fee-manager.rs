use anyhow::Result;
use fee_manager::config::get_config;
use fee_manager::run;
use sqlx::postgres::PgPoolOptions;
use std::{net::TcpListener, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = get_config().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string())
        .await
        .expect("can't connect to database");

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener, pool).await?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "fee_manager=trace".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}
