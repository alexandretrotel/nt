use crate::{cli::DryRun, services::networks::macos, utils::dry_run::DryRunable};
use anyhow::Result;

pub fn run(iface: &str, ssid: &Option<String>, dry_run: DryRun) -> Result<()> {
    let action = || {
        #[cfg(target_os = "macos")]
        {
            let ssid_ref = ssid.as_deref();
            macos::remove::remove_network_or_interactive(iface, ssid_ref, dry_run)?;
            Ok(())
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(anyhow::anyhow("Not supported on this platform"))
        }
    };

    let message = match ssid {
        Some(s) => format!("Would remove network '{}' from interface '{}'", s, iface),
        None => format!(
            "Would remove networks interactively from interface '{}'",
            iface
        ),
    };

    action.run_with_message(dry_run, &message)
}
