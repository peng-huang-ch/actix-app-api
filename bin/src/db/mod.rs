//! Database debugging tool

use clap::{Parser, Subcommand};
use srv_storage::{init_db, models::version::get_db_version, run_migrations};

/// `api db` command
#[derive(Debug, Parser)]
pub struct Command {
    #[arg(long, value_name = "database url", verbatim_doc_comment, default_value_t, global = true)]
    database_url: String,

    #[clap(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand, Debug)]
/// `reth db` subcommands
pub enum Subcommands {
    /// Execute database migrations
    Migration,
    /// Lists current and local database versions
    Version,
}

impl Command {
    /// Execute `db` command
    pub async fn execute(self) -> eyre::Result<()> {
        let database_url = self.database_url.as_str();

        match self.command {
            Subcommands::Migration { .. } => {
                let _migrated = run_migrations(database_url);
                println!("database migrations complete")
            }
            Subcommands::Version {} => {
                let pool = init_db(database_url).await;
                let mut conn = pool.get().await.expect("could not get connection");
                let version = get_db_version(&mut conn).await;
                println!("database version {}", version);
            }
        }
        Ok(())
    }
}
