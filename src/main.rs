mod configuration;

use crate::configuration::read_configuration;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;
use zero2prod::run_app;
use zero2prod::telemetry::{build_subscriber, setup_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = build_subscriber("zero2prod", "info", io::stdout);
    setup_subscriber(subscriber);

    let configuration = read_configuration().expect("Configuration should be red");

    let connection = PgPool::connect_with(configuration.database.build_connect_options())
        .await
        .expect("Postgres should connect");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    run_app(
        TcpListener::bind(address).expect("OS should bind listener"),
        connection,
    )?
    .await
}
