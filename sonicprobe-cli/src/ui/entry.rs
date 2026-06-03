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
            Some(unit) => format!("{:>9} {}", self.value, unit),
            None => self.value
        }
    }

    pub fn into_parts(self) -> (String, Option<String>) {
        (self.value, self.unit)
    }

    pub fn from_rate(label: &str) -> Self {
        Self {
            value: label.to_owned(),
            unit: Some(String::from("kHz")),
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
        format!("+{value:.2}")
    } else if value == 0.0 {
        "0.00".to_owned()
    } else {
        format!("{value:.2}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_formatted_no_padding() {
        assert_eq!(Entry::from(2usize).formatted(), "2");
    }

    #[test]
    fn usize_formatted_large_no_padding() {
        assert_eq!(Entry::from(15375024usize).formatted(), "15375024");
    }

    #[test]
    fn from_rate_parts() {
        assert_eq!(
            Entry::from_rate("44.1").into_parts(),
            ("44.1".to_owned(), Some("kHz".to_owned()))
        );
    }

    #[test]
    fn from_bit_parts() {
        assert_eq!(
            Entry::from_bit(16).into_parts(),
            ("16".to_owned(), Some("bit".to_owned()))
        );
    }

    #[test]
    fn usize_parts_no_unit() {
        assert_eq!(Entry::from(2usize).into_parts(), ("2".to_owned(), None));
    }
}
