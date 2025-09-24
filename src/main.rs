mod configuration;

use crate::configuration::read_configuration;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::run_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    LogTracer::init().expect("LogTracer should init");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Default subscriber should set");

    let configuration = read_configuration().expect("Configuration should be red");

    let connection = PgPool::connect(&configuration.database.format_connection_string())
        .await
        .expect("Postgres should connect");

    let address = format!("0.0.0.0:{}", configuration.application_port);

    run_app(
        TcpListener::bind(address).expect("OS should bind listener"),
        connection,
    )?
    .await
}
