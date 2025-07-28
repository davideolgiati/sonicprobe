use crate::ui::audio::{format_db, format_dr, format_hz, format_volt};

pub struct Entry {
        value: String,
        unit: String
}

impl Entry {
        pub fn formatted(self) -> String {
                format!("{} {:>4}", self.value, self.unit)
        }

        pub fn from_db(value: f32) -> Entry {
                Entry {
                        value: format_db(value),
                        unit: String::from("dB")
                }
        }

        pub fn from_volt(value: f32) -> Entry {
                Entry {
                        value: format_volt(value),
                        unit: String::from("V")
                }
        }

        pub fn from_hz(value: f32) -> Entry {
                let new_value = value.round() as u32;
                Entry { 
                        value: format_hz(new_value),
                        unit: String::from("Hz")
                }
        }

        pub fn from_percent(value: f32) -> Entry {
                Entry {
                        value: format_percent(value),
                        unit: String::from("%")
                }
        }

        pub fn from_dr(value: f32) -> Entry {
                let new_value = value.round() as u32;
                Entry {
                        value: format_dr(new_value),
                        unit: String::from("DR")
                }
        }

        pub fn from_bit(value: u8) -> Entry {
                Entry { 
                        value: format!("{}", value), 
                        unit: String::from("bit") 
                }
        }
}

pub fn format_percent(value: f32) -> String {
        if value > 0.0 {
            format!("+{:.5}", value * 100.0)
        } else if value == 0.0 {
            String::from("0.00000")
        } else {
            format!("-{:.5}", value * 100.0)
        }
}