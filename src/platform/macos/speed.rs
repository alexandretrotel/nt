use anyhow::Result;
use std::io::{Write, stdout};
use sysinfo::Networks;
use tokio::time::{Duration, sleep};

use crate::domain::speed::unit::{Unit, format_speed_normalized};

pub async fn realtime_speed(iface: String, unit: Unit, delay: u64) -> Result<()> {
    let mut prev_rx: u64 = 0;
    let mut prev_tx: u64 = 0;

    loop {
        let networks = Networks::new_with_refreshed_list();

        if let Some(data) = networks.get(&iface) {
            let rx = data.total_received();
            let tx = data.total_transmitted();

            if prev_rx != 0 {
                let down_b = rx.saturating_sub(prev_rx);
                let up_b = tx.saturating_sub(prev_tx);

                let down_str = format_speed_normalized(down_b, unit, delay);
                let up_str = format_speed_normalized(up_b, unit, delay);

                print!("\r↓ {:>10}  ↑ {:>10}", down_str, up_str);
                stdout().flush()?;
            }

            prev_rx = rx;
            prev_tx = tx;
        }

        sleep(Duration::from_millis(delay)).await;
    }
}

pub fn get_active_interface() -> Option<String> {
    use std::process::Command;

    let output = Command::new("route")
        .args(["get", "default"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("interface:") {
            return Some(rest.trim().to_string());
        }
    }
    None
}
