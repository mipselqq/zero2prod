use crate::{EmailClient, routes::subscribe};
use actix_web::{
    App, HttpServer,
    dev::Server,
    web::{self, Data},
};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::health_check;

pub fn run_app(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let email_client = Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
