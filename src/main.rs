mod configuration;

use std::net::TcpListener;
use zero2prod::run_app;

use crate::configuration::read_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = read_configuration().expect("Configuration should be red");
    let address = format!("0.0.0.0:{}", configuration.application_port);

    run_app(TcpListener::bind(address).expect("OS should bind listener"))?.await
}
