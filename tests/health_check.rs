use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};
use sqlx::{Connection, PgConnection, PgPool, query};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::read_configuration;
use zero2prod::telemetry::{build_subscriber, setup_subscriber};
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
    let TestApp { address, db_pool } = spawn_app().await;
    let client = Client::new();

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
        .fetch_one(&db_pool)
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

#[tokio::test]
async fn subscribe_returns_bad_request_for_present_empty_fields() {
    let TestApp { address, .. } = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
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
            "App didn't return BAD_REQUEST when payload was {error_message}"
        )
    }
}

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let s = build_subscriber("test", "info", std::io::stdout);
        setup_subscriber(s);
    } else {
        let s = build_subscriber("test", "info", std::io::sink);
        setup_subscriber(s);
    };
});

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener =
        TcpListener::bind("0.0.0.0:0").expect("OS should bind app listener to random port");
    let port = listener.local_addr().unwrap().port();

    let mut config = read_configuration().expect("Configuration should be read");
    config.database.name = Uuid::new_v4().to_string();

    let connection_pool = configure_db(config).await;
    let server = run_app(listener, connection_pool.clone()).expect("App should run");

    tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{port}"),
        db_pool: connection_pool,
    }
}

async fn configure_db(config: Settings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.database.build_connect_options_nodb())
        .await
        .expect("Postgres should connect");

    query(&format!(r#"CREATE DATABASE "{}""#, config.database.name))
        .execute(&mut connection)
        .await
        .expect("Database should be created");

    let pool = PgPool::connect_with(config.database.build_connect_options())
        .await
        .expect("Postgres should connect");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations should run");

    pool
}
