use crate::ui::{audio::{format_db, format_hz, format_volt}, format_percent};

pub struct Entry {
        value: String
}

impl Entry {
        pub fn new(value: &str) -> Entry {
                Entry { 
                        value : value.to_string()
                }
        }

        pub fn value(self) -> String {
                self.value
        }

        pub fn from_db(value: f32) -> Entry {
                Entry {
                        value: format_db(value)
                }
        }

        pub fn from_volt(value: f32) -> Entry {
                Entry {
                        value: format_volt(value)
                }
        }

        pub fn from_hz(value: f32) -> Entry {
                let new_value = value.round() as u32;
                Entry { 
                        value: format_hz(new_value) 
                }
        }

        pub fn from_percent(value: f32) -> Entry {
                Entry {
                        value: format_percent(value)
                }
        }
}