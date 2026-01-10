use anyhow::Result;
use std::process::Command;

#[cfg(target_os = "macos")]
use inquire::MultiSelect;

use crate::{cli::DryRun, utils::dry_run::DryRunable};

#[cfg(target_os = "macos")]
use super::{list, remove};

pub fn remove_network_or_interactive(
    iface: &str,
    ssid: Option<&str>,
    dry_run: DryRun,
) -> Result<()> {
    match ssid {
        Some(s) => remove_network(iface, s)?,
        None => remove_networks_interactive(iface, dry_run)?,
    }
    Ok(())
}

pub fn remove_networks_interactive(iface: &str, dry_run: DryRun) -> Result<()> {
    let networks = list::get_preferred_networks(iface)?;

    if networks.is_empty() {
        println!("No networks found on interface '{}'.", iface);
        return Ok(());
    }

    let chosen = MultiSelect::new("Select networks to remove", networks).prompt()?;

    if chosen.is_empty() {
        println!("No networks selected. Aborting.");
        return Ok(());
    }

    for ssid in chosen {
        let action = || remove::remove_network(iface, &ssid);
        action.run_with_message(
            dry_run,
            &format!("Would remove network '{}' from '{}'", ssid, iface),
        )?;
    }

    Ok(())
}

pub fn remove_network(iface: &str, ssid: &str) -> Result<()> {
    Command::new("networksetup")
        .args(["-removepreferredwirelessnetwork", iface, ssid])
        .status()?;

    println!("Removed network: {ssid}");
    Ok(())
}
