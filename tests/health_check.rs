use reqwest::{Client, StatusCode};
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::read_configuration;
use zero2prod::run_app;

#[tokio::test]
async fn health_check_returns_success() {
    let addr = spawn_app();

    let response = reqwest::get(format!("{addr}/health_check"))
        .await
        .expect("Request should execute");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_ok_for_valid_form_data() {
    let addr = spawn_app();
    let client = Client::new();

    let configuration = read_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.format_connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Postgres sould connect");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{addr}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request should execute");

    assert_eq!(StatusCode::OK, response.status());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Postgres should fetch");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscibe_returns_bad_request_for_missing_data() {
    let addr = spawn_app();
    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{addr}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Request should execute");

        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "App didn't fail with BAD_REQUEST when payload was {error_message}"
        )
    }
}

fn spawn_app() -> String {
    let listener =
        TcpListener::bind("0.0.0.0:0").expect("OS should bind app listener to random port");

    let port = listener.local_addr().unwrap().port();
    let server = run_app(listener).expect("App should run");

    tokio::spawn(server);

    format!("http://localhost:{port}")
}
