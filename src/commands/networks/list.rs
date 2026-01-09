use crate::{cli::DryRun, services::networks::macos};
use anyhow::Result;

pub fn run(iface: &str, dry_run: DryRun) -> Result<()> {
    if dry_run.enabled {
        println!("[dry-run] Would list networks on interface '{}'", iface);
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        macos::list_networks(iface)?;
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err(anyhow::anyhow("Not supported on this platform"))
    }
}
