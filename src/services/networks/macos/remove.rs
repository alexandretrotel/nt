use anyhow::{Context, Result};
use std::process::{Command, Stdio};

#[cfg(target_os = "macos")]
use inquire::MultiSelect;

#[cfg(target_os = "macos")]
use super::list;

use tokio::task::JoinSet;

pub async fn remove_network_or_interactive(iface: &str, ssid: Option<&str>) -> Result<()> {
    match ssid {
        Some(s) => {
            remove_network(iface, s)?;
            println!("✓ {}", s);
            Ok(())
        }
        None => remove_networks_interactive(iface).await,
    }
}

pub async fn remove_networks_interactive(iface: &str) -> Result<()> {
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

    let results = remove_networks_parallel(iface, chosen).await;

    let (ok, failed): (Vec<_>, Vec<_>) = results.into_iter().partition(|(_, r)| r.is_ok());

    println!();
    println!("Removed {} network(s) from {}", ok.len(), iface);
    for (ssid, _) in ok {
        println!("  ✓ {}", ssid);
    }

    if !failed.is_empty() {
        println!();
        println!("Failed to remove {} network(s):", failed.len());
        for (ssid, err) in failed {
            println!("  ✗ {} ({})", ssid, err.unwrap_err());
        }
    }

    Ok(())
}

async fn remove_networks_parallel(iface: &str, ssids: Vec<String>) -> Vec<(String, Result<()>)> {
    let mut set = JoinSet::new();

    for ssid in ssids {
        let iface = iface.to_string();
        set.spawn_blocking(move || {
            let res = remove_network(&iface, &ssid);
            (ssid, res)
        });
    }

    let mut results = Vec::new();

    while let Some(res) = set.join_next().await {
        if let Ok(pair) = res {
            results.push(pair);
        }
    }

    results
}

pub fn remove_network(iface: &str, ssid: &str) -> Result<()> {
    let status = Command::new("networksetup")
        .args(["-removepreferredwirelessnetwork", iface, ssid])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| format!("failed to execute networksetup for '{}'", ssid))?;

    if !status.success() {
        anyhow::bail!("network not found or removal failed");
    }

    Ok(())
}
