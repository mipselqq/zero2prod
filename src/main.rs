mod configuration;

use crate::configuration::read_configuration;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::run_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = build_subscriber("zero2prod", "info");
    setup_subscriber(subscriber);

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

pub fn build_subscriber(name: &str, env_filter: &str) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn setup_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("LogTracer should init");
    set_global_default(subscriber).expect("Default subscriber should set");
}
