use crate::cli::{Cli, Commands};
use clap::Parser;

use crate::constants::default_cli_args::DEFAULT_IFACE;

pub async fn dispatch() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Speed {
        iface: DEFAULT_IFACE.to_string(),
    }) {
        Commands::Speed { iface } => crate::commands::speed::run(iface).await?,
        Commands::Networks { action } => crate::commands::networks::run(action)?,
    }

    Ok(())
}
