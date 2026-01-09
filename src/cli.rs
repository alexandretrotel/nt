use clap::{Parser, Subcommand};

use crate::utils::network::Unit;

use crate::constants::default_cli_args::DEFAULT_IFACE;

#[derive(Parser)]
#[command(name = "nt")]
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
        #[arg(short, long, default_value_t = String::from(DEFAULT_IFACE))]
        iface: String,
        /// Unit to format speed (bps/kbps/mbps/gbps)
        #[arg(short, long, value_enum, default_value_t = Unit::Mbps)]
        unit: Unit,
    },
    /// Manage network interfaces
    Networks {
        #[command(subcommand)]
        action: NetworkAction,
    },
}

#[derive(Subcommand)]
pub enum NetworkAction {
    /// List network interfaces
    List {
        /// Interface name
        #[arg(short, long, default_value_t = String::from(DEFAULT_IFACE))]
        iface: String,
    },
    /// Remove a network interface
    Remove {
        /// Interface name
        #[arg(short, long, default_value_t = String::from(DEFAULT_IFACE))]
        iface: String,
        /// SSID of the network to remove
        ssid: String,
    },
}
