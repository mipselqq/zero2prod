use zero2prod::run_app;

#[tokio::test]
async fn health_check_responds_ok() {
    spawn_app();

    let response = reqwest::get("http://localhost:8000/health_check")
        .await
        .expect("Request should proceed");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = run_app().expect("App should run");
    tokio::spawn(server);
}
