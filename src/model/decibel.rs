use std::ops::Sub;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Decibel(f64);

impl Decibel {
        pub fn new(value: f64) -> Self {
                Self(to_dbfs(value))
        }

        pub const fn get_value(self) -> f64 {
                self.0
        }

        pub fn get_string_value(self) -> String {
                format(self.0)
        }

        pub fn get_unit() -> String {
                "dB".to_owned()
        }
}

impl Sub for Decibel {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

fn format(value: f64) -> String {
    if value > 0.0 {
        format!("+{value:.2}")
    } else if value == 0.0 {
        String::from("0.00")
    } else {
        format!("{value:.2}")
    }
}

fn to_dbfs(dc: f64) -> f64 {
    20.0 * dc.log10()
}