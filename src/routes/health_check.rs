use actix_web::{HttpResponse, Responder};
use tracing::info;

pub async fn health_check() -> impl Responder {
    info!("Received health check request");
    HttpResponse::Ok()
}
