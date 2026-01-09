use crate::{cli::DryRun, services::networks::macos};
use anyhow::Result;

pub fn run(iface: &str, ssid: &str, dry_run: DryRun) -> Result<()> {
    if dry_run.enabled {
        println!(
            "[dry-run] Would remove network '{}' from interface '{}'",
            ssid, iface
        );
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        macos::remove_network(iface, ssid)?;
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(anyhow::anyhow("Not supported on this platform"))
    }
}
