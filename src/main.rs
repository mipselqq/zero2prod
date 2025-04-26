use std::io;

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    zero2prod::run()?.await
}
