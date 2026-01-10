use anyhow::Result;
use std::process::Command;

pub fn get_preferred_networks(iface: &str) -> Result<Vec<String>> {
    let output = Command::new("networksetup")
        .args(["-listpreferredwirelessnetworks", iface])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<String> = stdout
        .lines()
        .skip(1)
        .filter(|line| !line.trim().chars().all(|c| c == '-'))
        .map(|s| s.trim().to_string())
        .collect();

    Ok(lines)
}

pub fn list_networks(iface: &str) -> Result<()> {
    let lines = get_preferred_networks(iface)?;
    let width = lines.len().to_string().len();

    println!("Preferred networks on {}", iface);
    for (i, ssid) in lines.iter().enumerate() {
        println!("{:>width$}. {}", i + 1, ssid, width = width);
    }

    Ok(())
}
