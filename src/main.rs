use anyhow::Result;
use fee_manager::configuration::get_configuration;
use fee_manager::run;
use std::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener).await?;

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
