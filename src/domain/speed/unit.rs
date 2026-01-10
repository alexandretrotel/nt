use clap::ValueEnum;
use std::{fmt, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Unit {
    Bps,
    Kbps,
    Mbps,
    Gbps,
}

impl Unit {
    pub fn divisor(self) -> f64 {
        match self {
            Unit::Bps => 1.0,
            Unit::Kbps => 1_000.0,
            Unit::Mbps => 1_000_000.0,
            Unit::Gbps => 1_000_000_000.0,
        }
    }

    pub fn suffix(self) -> &'static str {
        match self {
            Unit::Bps => "bps",
            Unit::Kbps => "Kbps",
            Unit::Mbps => "Mbps",
            Unit::Gbps => "Gbps",
        }
    }

    pub fn from_bits(self, bits: f64) -> f64 {
        bits / self.divisor()
    }
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bps" | "b" | "bit" | "bits" => Ok(Unit::Bps),
            "kbps" | "k" | "kbit" => Ok(Unit::Kbps),
            "mbps" | "m" | "mbit" => Ok(Unit::Mbps),
            "gbps" | "g" | "gbit" => Ok(Unit::Gbps),
            other => Err(format!("unknown unit: {other}")),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.suffix())
    }
}

pub fn parse_unit(s: &str) -> Option<Unit> {
    s.parse().ok()
}

pub fn bytes_to_unit(bytes: u64, unit: Unit) -> f64 {
    let bits = bytes as f64 * 8.0;
    unit.from_bits(bits)
}

pub fn format_speed(bytes: u64, unit: Unit) -> String {
    format!("{:.2} {}", bytes_to_unit(bytes, unit), unit)
}

pub fn format_speed_normalized(bytes: u64, unit: Unit, interval_ms: u64) -> String {
    if interval_ms == 0 {
        return format_speed(bytes, unit);
    }

    let bits_per_sec = (bytes as f64 * 8.0) * (1000.0 / interval_ms as f64);

    format!("{:.2} {}", unit.from_bits(bits_per_sec), unit)
}
