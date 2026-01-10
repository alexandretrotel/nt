use crate::{cli::DryRun, utils::dry_run::DryRunable};
use anyhow::Result;

#[cfg(target_os = "macos")]
pub fn run(iface: &str, dry_run: DryRun) -> Result<()> {
    let action = || {
        crate::platform::macos::networks::list::list_networks(iface)?;
        Ok(())
    };

    action.run_with_message(
        dry_run,
        &format!("Would list networks on interface '{}'", iface),
    )
}

#[cfg(not(target_os = "macos"))]
pub fn run(_iface: &str, dry_run: DryRun) -> Result<()> {
    let action = || Err(anyhow::anyhow!("Not supported on this platform"));

    action.run_with_message(
        dry_run,
        &format!("Would list networks on interface '{}'", _iface),
    )
}
