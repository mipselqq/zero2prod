use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let adress = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{adress}/health_check"))
        .send()
        .await
        .expect("Test request should execute");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Adress should bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Adress should be bound correctly");

    let _ = actix_rt::spawn(server);

    format!("http://localhost:{port}")
}
