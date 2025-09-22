use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    tracing::info!("Received health check request");
    HttpResponse::Ok()
}
