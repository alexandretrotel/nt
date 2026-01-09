use anyhow::Result;
use std::io::{Write, stdout};
use sysinfo::Networks;
use tokio::time::{Duration, sleep};

use crate::utils::network::{Unit, format_speed};

pub async fn realtime_speed(iface: String) -> Result<()> {
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

                let down_str = format_speed(down_b, Unit::Mbps);
                let up_str = format_speed(up_b, Unit::Mbps);

                print!("\r↓ {:>10}  ↑ {:>10}", down_str, up_str);
                stdout().flush()?;
            }

            prev_rx = rx;
            prev_tx = tx;
        }

        sleep(Duration::from_secs(1)).await;
    }
}
