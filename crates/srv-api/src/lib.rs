#![feature(async_closure)]
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use tokio::sync::oneshot;
use tracing::{debug, warn};
use tracing_actix_web::TracingLogger;

mod database;
mod errors;
mod handles;
mod models;
mod schema;
mod shutdown;
use database::init_db_pool;
use handles::{add_signature, query_signature};
use shutdown::shutdown;
use srv_tracing::init_logging;
use std::time::Duration;

pub async fn init() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let guard = init_logging("app".to_string(), "debug".to_string());

    debug!(target: "init", "Initializing database...");
    let pool = init_db_pool();
    debug!(target: "init", "Database connected.");

    let srv: actix_web::dev::Server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            .service(add_signature)
            .service(query_signature)
    })
    .disable_signals()
    .bind("127.0.0.1:8090")?
    .run();

    let srv_handle = srv.handle();
    let server_task = tokio::spawn(srv);

    let shutdown_handle = shutdown(async move {
        srv_handle.stop(true).await;
        drop(guard);
        let (tx, rx) = oneshot::channel();
        tokio::task::spawn_blocking(|| {
            debug!("shutting down the tracer provider.");
            opentelemetry::global::shutdown_tracer_provider();
            debug!("shutdown the tracer provider.");
            let _ = tx.send(());
        })
        .await
        .expect("shutdown tracer provider failed.");

        // Wrap the future with a `Timeout` set to expire in 10 seconds.
        if let Err(_) = tokio::time::timeout(Duration::from_secs(10), rx).await {
            warn!("timed out while shutting down tracing, exiting anyway");
        };
    });

    let shutdown_task = tokio::spawn(shutdown_handle);
    let _ = tokio::try_join!(server_task, shutdown_task).expect("unable to join tasks");

    Ok(())
}
