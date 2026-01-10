use clap::ValueEnum;

pub fn bytes_to_mbps(bytes: u64) -> f64 {
    (bytes as f64 * 8.0) / 1_000_000.0
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Unit {
    Bps,
    Kbps,
    Mbps,
    Gbps,
}

impl std::str::FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bps" | "b" | "bit" | "bits" => Ok(Unit::Bps),
            "kbps" | "k" | "kbit" => Ok(Unit::Kbps),
            "mbps" | "m" | "mbit" => Ok(Unit::Mbps),
            "gbps" | "g" | "gbit" => Ok(Unit::Gbps),
            other => Err(format!("unknown unit: {}", other)),
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Unit::Bps => "bps",
            Unit::Kbps => "Kbps",
            Unit::Mbps => "Mbps",
            Unit::Gbps => "Gbps",
        };
        write!(f, "{}", s)
    }
}

pub fn parse_unit(s: &str) -> Option<Unit> {
    s.parse::<Unit>().ok()
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

pub fn format_speed_normalized(bytes: u64, unit: Unit, interval_ms: u64) -> String {
    if interval_ms == 0 {
        return format_speed(bytes, unit);
    }

    let bytes_per_sec = (bytes as f64) * (1000.0f64 / interval_ms as f64);
    let bits_per_sec = bytes_per_sec * 8.0;

    let value = match unit {
        Unit::Bps => bits_per_sec,
        Unit::Kbps => bits_per_sec / 1_000.0,
        Unit::Mbps => bits_per_sec / 1_000_000.0,
        Unit::Gbps => bits_per_sec / 1_000_000_000.0,
    };

    format!("{:.2} {}", value, unit)
}

#[cfg(target_os = "macos")]
pub fn get_active_interface() -> Option<String> {
    use std::process::Command;

    let output = Command::new("route")
        .args(["get", "default"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("interface:") {
            return Some(rest.trim().to_string());
        }
    }
    None
}
