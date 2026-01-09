use crate::services::speed;
use anyhow::Result;

pub async fn run(iface: String) -> Result<()> {
    speed::realtime_speed(iface).await
}
