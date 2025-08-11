use std::net::TcpListener;

use zero2prod::run_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run_app(TcpListener::bind("0.0.0.0:8000").expect("OS should bind listener"))?.await
}
