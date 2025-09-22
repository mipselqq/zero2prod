use reqwest::{Client, StatusCode};
use sqlx::{Connection, PgConnection, PgPool, query};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::read_configuration;
use zero2prod::{Settings, run_app};

#[tokio::test]
async fn health_check_returns_success() {
    let TestApp { address, .. } = spawn_app().await;

    let response = reqwest::get(format!("{address}/health_check"))
        .await
        .expect("Request should execute");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_ok_for_valid_form_data() {
    let TestApp { address, .. } = spawn_app().await;
    let client = Client::new();

    let configuration = read_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.format_connection_string();

    let connection = PgPool::connect(&connection_string)
        .await
        .expect("Postgres should connect");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request should execute");

    assert_eq!(StatusCode::OK, response.status());

    let saved = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&connection)
        .await
        .expect("Postgres should fetch");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_bad_request_for_missing_data() {
    let TestApp { address, .. } = spawn_app().await;

    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
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

struct TestApp {
    pub address: String,
    pub _db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind("0.0.0.0:0").expect("OS should bind app listener to random port");
    let port = listener.local_addr().unwrap().port();

    let mut config = read_configuration().expect("Configuration should be read");
    config.database.name = Uuid::new_v4().to_string();

    let connection_pool = configure_db(config).await;
    let server = run_app(listener, connection_pool.clone()).expect("App should run");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{port}"),
        _db_pool: connection_pool,
    }
}

async fn configure_db(config: Settings) -> PgPool {
    let mut connection =
        PgConnection::connect(&config.database.format_connection_string_without_db())
            .await
            .expect("Postgres should connect");

    query(&format!(r#"CREATE DATABASE "{}""#, config.database.name))
        .execute(&mut connection)
        .await
        .expect("Database should be created");

    let pool = PgPool::connect(&config.database.format_connection_string())
        .await
        .expect("Postgres should connect");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations should run");

    pool
}
