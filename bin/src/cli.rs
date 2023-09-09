//! CLI definition and entrypoint to executable
use crate::{
    api, db,
    version::{LONG_VERSION, SHORT_VERSION},
};
use clap::{Args, Parser, Subcommand};

/// Parse CLI options, set up logging and run the chosen command.
pub async fn run() -> eyre::Result<()> {
    let opt = Cli::parse();
    // reth_tracing::init(layers);
    match opt.command {
        Commands::Db(command) => command.execute().await?,
        Commands::Api(command) => command.execute().await?,
    };
    Ok(())
}

/// Commands to be executed
#[derive(Debug, Subcommand)]
pub enum Commands {
    // /// Start the node
    // #[command(name = "node")]
    // Node(node::Command),
    // /// Initialize the database from a genesis file.
    // #[command(name = "init")]
    // Init(chain::InitCommand),
    // /// This syncs RLP encoded blocks from a file.
    // #[command(name = "import")]
    // Import(chain::ImportCommand),
    // /// Database debugging utilities
    #[command(name = "db")]
    Db(db::Command),

    #[command(name = "api")]
    Api(api::Command),
}

#[derive(Debug, Parser)]
#[command(author, version = SHORT_VERSION, long_version = LONG_VERSION, about = "app", long_about = None)]
pub struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Commands,
}

/// The log configuration.
#[derive(Debug, Args)]
#[command(next_help_heading = "Logging")]
pub struct Logs {
    /// Log events to journald.
    #[arg(long = "log.journald", global = true, conflicts_with = "log_directory")]
    journald: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    /// Tests that the help message is parsed correctly. This ensures that clap args are configured
    /// correctly and no conflicts are introduced via attributes that would result in a panic at
    /// runtime
    #[test]
    fn test_parse_help_all_subcommands() {
        let reth = Cli::command();
        for sub_command in reth.get_subcommands() {
            let err = Cli::try_parse_from(["reth", sub_command.get_name(), "--help"])
                .err()
                .unwrap_or_else(|| {
                    panic!("Failed to parse help message {}", sub_command.get_name())
                });

            // --help is treated as error, but
            // > Not a true "error" as it means --help or similar was used. The help message will be sent to stdout.
            assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
        }
    }
}
