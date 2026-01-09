use crate::services::networks::macos;
use anyhow::Result;

pub fn run(iface: &str) -> Result<()> {
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
