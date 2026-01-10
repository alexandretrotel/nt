use crate::cli::{Cli, Commands};
use crate::utils::network::Unit;
use clap::Parser;

use crate::constants::default_cli_args::default_iface;

pub async fn dispatch() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Speed {
        iface: default_iface(),
        unit: Unit::Mbps,
    }) {
        Commands::Speed { iface, unit } => crate::commands::speed::run(iface, unit).await?,
        Commands::Networks { action } => crate::commands::networks::run(action).await?,
    }

    Ok(())
}
