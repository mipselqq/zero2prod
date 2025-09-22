mod configuration;

use crate::configuration::read_configuration;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::run_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = read_configuration().expect("Configuration should be red");
    let connection = PgConnection::connect(&configuration.database.format_connection_string())
        .await
        .expect("Postgres should connect");

    let address = format!("0.0.0.0:{}", configuration.application_port);

    run_app(
        TcpListener::bind(address).expect("OS should bind listener"),
        connection,
    )?
    .await
}
