use anyhow::Result;
use std::process::Command;

pub fn remove_network(iface: &str, ssid: &str) -> Result<()> {
    Command::new("networksetup")
        .args(["-removepreferredwirelessnetwork", iface, ssid])
        .status()?;

    println!("Removed network: {ssid}");
    Ok(())
}
