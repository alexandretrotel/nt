use anyhow::Result;
use std::process::Command;

pub fn list_networks(iface: &str) -> Result<()> {
    let output = Command::new("networksetup")
        .args(["-listpreferredwirelessnetworks", iface])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{stdout}");
    Ok(())
}

pub fn remove_network(iface: &str, ssid: &str) -> Result<()> {
    Command::new("networksetup")
        .args(["-removepreferredwirelessnetwork", iface, ssid])
        .status()?;

    println!("Removed network: {ssid}");
    Ok(())
}
