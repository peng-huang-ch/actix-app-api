#[tokio::main]
async fn main() {
    if let Err(err) = actix_app_api::cli::run().await {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
