use crate::ui::audio::{format_db, format_dr, format_hz, format_volt};

pub struct Entry {
    value: String,
    unit: Option<String>,
}

impl Entry {
    pub fn formatted(self) -> String {
        match self.unit {
            Some(unit) => format!("{} {:>4}", self.value, unit),
            None => self.value
        }
        
    }

    pub const fn from_str(value: String) -> Self {
        Self {
            value,
            unit: None,
        }
    }

    pub fn from_usize(value: usize) -> Self {
        Self {
            value: format!("{value}"),
            unit: None,
        }
    }

    pub fn from_db(value: f64) -> Self {
        Self {
            value: format_db(value),
            unit: Some(String::from("dB")),
        }
    }

    pub fn from_volt(value: f64) -> Self {
        Self {
            value: format_volt(value),
            unit: Some(String::from("V")),
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_hz(value: u64) -> Self {
        Self {
            value: format_hz(value),
            unit: Some(String::from("Hz")),
        }
    }

    pub fn from_percent(value: f64) -> Self {
        Self {
            value: format_percent(value),
            unit: Some(String::from("%"))
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_dr(value: f64) -> Self {
        let new_value = value.round().abs() as i64;
        Self {
            value: format_dr(new_value),
            unit: Some(String::from("DR")),
        }
    }

    pub fn from_bit(value: u8) -> Self {
        Self {
            value: format!("{value}"),
            unit: Some(String::from("bit")),
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
