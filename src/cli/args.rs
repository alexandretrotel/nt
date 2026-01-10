use clap::{Args, Parser, Subcommand};

use crate::cli::defaults::default_iface;
use crate::domain::speed::unit::Unit;

#[derive(Parser)]
#[command(name = "ntw")]
#[command(about = "Network tooling for macOS", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display real-time network speed
    Speed {
        /// Interface name
        #[arg(short, long, default_value_t = default_iface())]
        iface: String,

        /// Unit to format speed (bps/kbps/mbps/gbps/auto)
        #[arg(short, long, value_enum, default_value_t = Unit::Auto)]
        unit: Unit,

        /// Delay between updates (in milliseconds)
        #[arg(short, long, default_value_t = 1000)]
        delay: u64,
    },

    /// List network interfaces
    List {
        /// Interface name
        #[arg(short, long, default_value_t = default_iface())]
        iface: String,

        #[command(flatten)]
        dry_run: DryRun,
    },

    /// Remove a network interface
    Remove {
        /// Interface name
        #[arg(short, long, default_value_t = default_iface())]
        iface: String,

        /// SSID of the network to remove
        #[arg(short, long)]
        ssid: Option<String>,

        #[command(flatten)]
        dry_run: DryRun,
    },
}

#[derive(Args, Debug, Clone, Copy)]
pub struct DryRun {
    /// Do not execute actions; show what would happen
    #[arg(long = "dry-run")]
    pub enabled: bool,
}
