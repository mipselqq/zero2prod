use std::{io, net::TcpListener};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    zero2prod::run(TcpListener::bind("localhost:8000").expect("Adress should bind"))?.await
}
