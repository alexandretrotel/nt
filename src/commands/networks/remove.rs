use crate::{cli::DryRun, services::networks::macos, utils::dry_run::DryRunable};
use anyhow::Result;

pub fn run(iface: &str, ssid: &str, dry_run: DryRun) -> Result<()> {
    let action = || {
        #[cfg(target_os = "macos")]
        {
            macos::remove::remove_network(iface, ssid)?;
            Ok(())
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(anyhow::anyhow("Not supported on this platform"))
        }
    };

    action.run_with_message(
        dry_run,
        &format!("Would remove network '{}' from interface '{}'", ssid, iface),
    )
}
