#[actix_web::main]
async fn main() {
    let _ = srv_api::init_api().await;
}
