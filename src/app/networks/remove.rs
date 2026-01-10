use crate::{cli::DryRun, utils::dry_run::run_async};
use anyhow::Result;

#[cfg(target_os = "macos")]
pub async fn run(iface: &str, ssid: Option<&str>, dry_run: DryRun) -> Result<()> {
    let message = match ssid {
        Some(s) => format!("Would remove network '{}' from interface '{}'", s, iface),
        None => format!(
            "Would remove networks interactively from interface '{}'",
            iface
        ),
    };

    run_async(dry_run, &message, || async {
        let ssid_ref = ssid.as_deref();
        crate::platform::macos::networks::remove::remove_network_or_interactive(iface, ssid_ref)
            .await
    })
    .await
}

#[cfg(not(target_os = "macos"))]
pub async fn run(_iface: &str, _ssid: Option<&str>, dry_run: DryRun) -> Result<()> {
    let message = match _ssid {
        Some(s) => format!("Would remove network '{}' from interface '{}'", s, _iface),
        None => format!(
            "Would remove networks interactively from interface '{}'",
            _iface
        ),
    };

    run_async(dry_run, &message, || async {
        Err(anyhow::anyhow!("Not supported on this platform"))
    })
    .await
}
