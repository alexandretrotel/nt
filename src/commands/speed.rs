use crate::services::speed;
use crate::utils::network::Unit;
use anyhow::Result;

pub async fn run(iface: String, unit: Unit) -> Result<()> {
    speed::realtime_speed(iface, unit).await
}
