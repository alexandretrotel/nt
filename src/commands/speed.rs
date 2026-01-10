use crate::services::speed;
use crate::utils::network::Unit;
use anyhow::Result;

pub async fn run(iface: String, unit: Unit, delay: u64) -> Result<()> {
    speed::realtime_speed(iface, unit, delay).await
}
