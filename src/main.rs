use sqlx::PgPool;
use std::io;
use std::net::TcpListener;
use zero2prod::telemetry::{build_subscriber, setup_subscriber};
use zero2prod::{EmailClient, read_configuration, run_app};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = build_subscriber("zero2prod", "info", io::stdout);
    setup_subscriber(subscriber);

    let configuration = read_configuration().expect("Configuration should be red");

    let connection = PgPool::connect_with(configuration.database.build_connect_options())
        .await
        .expect("Postgres should connect");
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    run_app(
        TcpListener::bind(address).expect("OS should bind listener"),
        connection,
        email_client,
    )?
    .await
}
