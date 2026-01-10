use crate::cli::defaults::default_iface;
use crate::cli::{Cli, Commands};
use crate::domain::speed::unit::Unit;
use clap::Parser;

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Speed {
        iface: default_iface(),
        unit: Unit::Auto,
        delay: 1000,
    }) {
        Commands::Speed { iface, unit, delay } => {
            crate::app::speed::run(iface, unit, delay).await?
        }
        Commands::List { iface, dry_run } => {
            crate::app::networks::run_list(&iface, dry_run)?;
        }
        Commands::Remove {
            iface,
            ssid,
            dry_run,
        } => {
            crate::app::networks::run_remove(&iface, &ssid, dry_run).await?;
        }
    }

    Ok(())
}
