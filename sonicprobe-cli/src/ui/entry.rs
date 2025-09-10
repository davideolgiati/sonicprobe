use sonicprobe_core::{decibel::Decibel, dynamic_range::DynamicRange};

use crate::ui::audio::{format_hz, format_volt};

pub struct Entry {
    value: String,
    unit: Option<String>,
}

impl From<String> for Entry {
    fn from(value: String) -> Self {
        Self {
            value,
            unit: None,
        }
    }
}

impl From<usize> for Entry {
    fn from(value: usize) -> Self {
        Self {
            value: format!("{value}"),
            unit: None,
        }
    }
}

impl From<Decibel> for Entry {
    fn from(obj: Decibel) -> Self {
        Self {
            value: obj.get_string_value(),
            unit: Some(Decibel::get_unit()),
        }
    }
}

impl From<DynamicRange> for Entry {
    fn from(obj: DynamicRange) -> Self {
        Self {
            value: obj.get_string_value(),
            unit: Some(DynamicRange::get_unit()),
        }
    }
}

impl Entry {
    pub fn formatted(self) -> String {
        match self.unit {
            Some(unit) => format!("{} {:>4}", self.value, unit),
            None => self.value
        }
    }

    pub fn from_volt(value: f64) -> Self {
        Self {
            value: format_volt(value),
            unit: Some(String::from("V")),
        }
    }

    pub fn from_hz(value: usize) -> Self {
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
