extern crate diesel;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use tracing::debug;

use std::io;
use tracing_actix_web::TracingLogger;

mod database;
mod errors;
mod handles;
mod logging;
mod models;
mod schema;

use database::init_db_pool;
use handles::{add_signature, query_signature};
use logging::init_logging;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().expect(".env file not found");
    init_logging();

    debug!(target: "main::app", "Initializing database...");
    let pool = init_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            // .service(web::resource("/users").route(web::post().to(add_signature())))
            .service(add_signature)
            .service(query_signature)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // Ensure all spans have been shipped to Jaeger.
    // opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
