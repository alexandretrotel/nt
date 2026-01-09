pub fn bytes_to_mbps(bytes: u64) -> f64 {
    (bytes as f64 * 8.0) / 1_000_000.0
}

#[derive(Copy, Clone)]
pub enum Unit {
    Bps,
    Kbps,
    Mbps,
    Gbps,
}

pub fn bytes_to_unit(bytes: u64, unit: Unit) -> f64 {
    let bits = bytes as f64 * 8.0;
    match unit {
        Unit::Bps => bits,
        Unit::Kbps => bits / 1_000.0,
        Unit::Mbps => bits / 1_000_000.0,
        Unit::Gbps => bits / 1_000_000_000.0,
    }
}

pub fn format_speed(bytes: u64, unit: Unit) -> String {
    let value = bytes_to_unit(bytes, unit);
    let suffix = match unit {
        Unit::Bps => "bps",
        Unit::Kbps => "Kbps",
        Unit::Mbps => "Mbps",
        Unit::Gbps => "Gbps",
    };
    format!("{:.2} {}", value, suffix)
}
