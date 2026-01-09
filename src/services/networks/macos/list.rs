use anyhow::Result;
use std::process::Command;

pub fn list_networks(iface: &str) -> Result<()> {
    let output = Command::new("networksetup")
        .args(["-listpreferredwirelessnetworks", iface])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let lines: Vec<&str> = stdout
        .lines()
        .skip(1)
        .filter(|line| !line.trim().chars().all(|c| c == '-'))
        .collect();
    let width = lines.len().to_string().len();

    println!("Preferred networks on {}", iface);
    for (i, ssid) in lines.iter().enumerate() {
        let trimmed = ssid.trim_start();
        println!("{:>width$}. {}", i + 1, trimmed, width = width);
    }

    Ok(())
}
