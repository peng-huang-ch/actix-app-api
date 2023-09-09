//! Main node command
//!
//! Starts the client
use crate::version::SHORT_VERSION;
use clap::Parser;

use srv_api::init_api;
use srv_tracing::tracing::info;
use std::{net::SocketAddr, path::PathBuf};

/// Start the node
#[derive(Debug, Parser)]
pub struct Command {
    #[arg(long, value_name = "database url", verbatim_doc_comment, global = true)]
    database_url: Option<String>,

    /// The path to the configuration file to use.
    #[arg(long, value_name = "FILE", verbatim_doc_comment)]
    config: Option<PathBuf>,

    /// Enable Prometheus metrics.
    ///
    /// The metrics will be served at the given interface and port.
    #[arg(long, value_name = "SOCKET", help_heading = "Metrics")]
    metrics: Option<SocketAddr>,
}

impl Command {
    /// Execute `node` command
    pub async fn execute(self) -> eyre::Result<()> {
        dotenvy::dotenv().ok();
        let _ = std::env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
        info!(target: "app::cli", "app {} starting", SHORT_VERSION);
        let _ = init_api().await?;
        info!(target: "reth::cli", "app has exited.");
        Ok(())
    }
}
