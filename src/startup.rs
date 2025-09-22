use crate::routes::subscribe;
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::health_check;

pub fn run_app(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
