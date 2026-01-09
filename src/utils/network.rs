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
