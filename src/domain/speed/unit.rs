use clap::ValueEnum;
use std::{fmt, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Unit {
    Auto,
    Bps,
    Kbps,
    Mbps,
    Gbps,
}

impl Unit {
    pub fn divisor(self) -> f64 {
        match self {
            Unit::Auto => 1.0,
            Unit::Bps => 1.0,
            Unit::Kbps => 1_000.0,
            Unit::Mbps => 1_000_000.0,
            Unit::Gbps => 1_000_000_000.0,
        }
    }

    pub fn suffix(self) -> &'static str {
        match self {
            Unit::Auto => "auto",
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
            "auto" => Ok(Unit::Auto),
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

pub fn choose_unit_from_bits(bits_per_sec: f64) -> Unit {
    if bits_per_sec >= 1_000_000_000.0 {
        Unit::Gbps
    } else if bits_per_sec >= 1_000_000.0 {
        Unit::Mbps
    } else if bits_per_sec >= 1_000.0 {
        Unit::Kbps
    } else {
        Unit::Bps
    }
}

pub fn parse_unit(s: &str) -> Option<Unit> {
    s.parse().ok()
}

pub fn bytes_to_unit(bytes: u64, unit: Unit) -> f64 {
    let bits = bytes as f64 * 8.0;
    unit.from_bits(bits)
}

fn format_value_by_unit(value: f64, unit: Unit) -> String {
    let number = match unit {
        Unit::Bps => format!("{:.0}", value),
        Unit::Kbps => format!("{:.0}", value),
        Unit::Mbps => {
            if value < 10.0 {
                format!("{:.2}", value)
            } else if value < 100.0 {
                format!("{:.1}", value)
            } else {
                format!("{:.0}", value)
            }
        }
        Unit::Gbps => {
            if value < 10.0 {
                format!("{:.2}", value)
            } else if value < 100.0 {
                format!("{:.1}", value)
            } else {
                format!("{:.2}", value)
            }
        }
        Unit::Auto => format!("{:.2}", value),
    };

    format!("{} {}", number, unit)
}

pub fn format_speed(bytes: u64, unit: Unit) -> String {
    let bits = bytes as f64 * 8.0;

    if unit == Unit::Auto {
        let chosen = choose_unit_from_bits(bits);
        let val = chosen.from_bits(bits);
        return format_value_by_unit(val, chosen);
    }

    let val = bytes_to_unit(bytes, unit);
    format_value_by_unit(val, unit)
}

pub fn format_speed_normalized(bytes: u64, unit: Unit, interval_ms: u64) -> String {
    if interval_ms == 0 {
        return format_speed(bytes, unit);
    }

    let bits_per_sec = (bytes as f64 * 8.0) * (1000.0 / interval_ms as f64);

    if unit == Unit::Auto {
        let chosen = choose_unit_from_bits(bits_per_sec);
        return format_value_by_unit(chosen.from_bits(bits_per_sec), chosen);
    }

    format_value_by_unit(unit.from_bits(bits_per_sec), unit)
}
