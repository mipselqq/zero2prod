use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run_app(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
