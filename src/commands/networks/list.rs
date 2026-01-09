use crate::{cli::DryRun, services::networks::macos, utils::dry_run::DryRunable};
use anyhow::Result;

pub fn run(iface: &str, dry_run: DryRun) -> Result<()> {
    let action = || {
        #[cfg(target_os = "macos")]
        {
            macos::list::list_networks(iface)?;
            Ok(())
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(anyhow::anyhow("Not supported on this platform"))
        }
    };

    action.run_with_message(
        dry_run,
        &format!("Would list networks on interface '{}'", iface),
    )
}
