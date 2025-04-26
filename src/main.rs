use std::io;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    run().await
}
