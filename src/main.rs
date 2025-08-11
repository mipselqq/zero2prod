use zero2prod::run_app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run_app()?.await
}
