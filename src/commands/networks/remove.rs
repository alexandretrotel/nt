use crate::services::networks::macos;
use anyhow::Result;

pub fn run(iface: &str, ssid: &str) -> Result<()> {
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
