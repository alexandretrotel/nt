use crate::domain::speed::unit::Unit;
use anyhow::Result;

#[cfg(target_os = "macos")]
pub async fn run(iface: &str, unit: Unit, delay: u64) -> Result<()> {
    crate::platform::macos::speed::realtime_speed(iface, unit, delay).await
}

#[cfg(not(target_os = "macos"))]
pub async fn run(_iface: String, _unit: Unit, _delay: u64) -> Result<()> {
    Err(anyhow::anyhow!(
        "Speed monitoring is only supported on macOS"
    ))
}
