use crate::{cli::DryRun, services::networks::macos, utils::dry_run::run_async};
use anyhow::Result;

pub async fn run(iface: &str, ssid: &Option<String>, dry_run: DryRun) -> Result<()> {
    let message = match ssid {
        Some(s) => format!("Would remove network '{}' from interface '{}'", s, iface),
        None => format!(
            "Would remove networks interactively from interface '{}'",
            iface
        ),
    };

    run_async(dry_run, &message, || async {
        #[cfg(target_os = "macos")]
        {
            let ssid_ref = ssid.as_deref();
            macos::remove::remove_network_or_interactive(iface, ssid_ref).await
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(anyhow::anyhow("Not supported on this platform"))
        }
    })
    .await
}
