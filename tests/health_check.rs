use std::net::TcpListener;

use zero2prod::run_app;

#[tokio::test]
async fn health_check_responds_ok() {
    let addr = spawn_app();

    let response = reqwest::get(format!("{addr}/health_check"))
        .await
        .expect("Request should proceed");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener =
        TcpListener::bind("0.0.0.0:0").expect("OS should bind app listener to random port");

    let port = listener.local_addr().unwrap().port();
    let server = run_app(listener).expect("App should run");

    tokio::spawn(server);

    format!("http://localhost:{port}")
}
