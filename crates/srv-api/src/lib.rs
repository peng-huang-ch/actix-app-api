#![feature(async_closure)]

use actix_web::{web, App, HttpServer};
use srv_storage::init_db;
use tokio::sync::oneshot;
use tracing::{debug, warn};
use tracing_actix_web::TracingLogger;

#[cfg(feature = "async")]
mod async_handles;

#[cfg(feature = "sync")]
mod handles;

mod errors;
mod shutdown;

#[cfg(feature = "async")]
use crate::async_handles::{add_signature, query_signature};

#[cfg(feature = "sync")]
use crate::handles::{add_signature, query_signature};

use shutdown::shutdown;
use srv_tracing::init_logging;
use std::time::Duration;

pub async fn init() -> std::io::Result<()> {
    dotenvy::dotenv().expect(".env file not found");
    let guard = init_logging("app".to_string(), "debug".to_string());

    debug!(target: "init", "Initializing database...");
    let database_url = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
    #[cfg(feature = "sync")]
    let pool = init_db(database_url.as_str());

    #[cfg(feature = "async")]
    let pool = init_db(database_url.as_str()).await;
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
