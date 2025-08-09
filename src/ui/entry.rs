use crate::ui::audio::{format_db, format_dr, format_hz, format_volt};

pub struct Entry {
    value: String,
    unit: String,
}

impl Entry {
    pub fn formatted(self) -> String {
        format!("{} {:>4}", self.value, self.unit)
    }

    pub fn from_db(value: f64) -> Self {
        Self {
            value: format_db(value),
            unit: String::from("dB"),
        }
    }

    pub fn from_volt(value: f64) -> Self {
        Self {
            value: format_volt(value),
            unit: String::from("V"),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_hz(value: f64) -> Self {
        let new_value = value.round() as i64;
        Self {
            value: format_hz(new_value),
            unit: String::from("Hz"),
        }
    }

    pub fn from_percent(value: f64) -> Self {
        Self {
            value: format_percent(value),
            unit: String::from("%"),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_dr(value: f64) -> Self {
        let new_value = value.round().abs() as i64;
        Self {
            value: format_dr(new_value),
            unit: String::from("DR"),
        }
    }

    pub fn from_bit(value: u8) -> Self {
        Self {
            value: format!("{value}"),
            unit: String::from("bit"),
        }
    }
}

pub fn format_percent(value: f64) -> String {
    if value > 0.0 {
        format!("+{value:.5}")
    } else if value == 0.0 {
        format!("{value:.5}")
    } else {
        format!("-{value:.5}")
    }
}
