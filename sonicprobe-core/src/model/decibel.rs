use std::ops::Sub;

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Decibel(f64);

impl Decibel {
        #[must_use] pub fn new(value: f64) -> Self {
                Self(to_dbfs(value))
        }

        #[must_use] pub const fn get_value(self) -> f64 {
                self.0
        }

        #[must_use] pub fn get_string_value(self) -> String {
                format(self.0)
        }

        #[must_use] pub fn get_unit() -> String {
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

#[cfg(test)]
#[allow(clippy::float_cmp)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_converts_to_dbfs() {
        let db = Decibel::new(1.0);
        assert_eq!(db.get_value(), 0.0);
    }

    #[test]
    fn get_value() {
        let db = Decibel::new(0.1);
        assert_eq!(db.get_value(), -20.0);
    }

    #[test]
    fn get_string_value_positive() {
        let db = Decibel::new(2.0);
        assert_eq!(db.get_string_value(), "+6.02");
    }

    #[test]
    fn get_string_value_zero() {
        let db = Decibel::new(1.0);
        assert_eq!(db.get_string_value(), "0.00");
    }

    #[test]
    fn get_string_value_negative() {
        let db = Decibel::new(0.5);
        assert_eq!(db.get_string_value(), "-6.02");
    }

    #[test]
    fn get_unit() {
        assert_eq!(Decibel::get_unit(), "dB");
    }

    #[test]
    fn sub_operation() {
        let db1 = Decibel::new(1.0);
        let db2 = Decibel::new(0.5);
        let result = db1 - db2;
        assert!((result.get_value() - 6.02).abs() < 0.01);
    }

    #[test]
    fn serialize() {
        let db = Decibel::new(0.1);
        let json = serde_json::to_string(&db).unwrap();
        assert_eq!(json, "-20.0");
    }

    #[test]
    fn format_positive() {
        assert_eq!(format(6.123), "+6.12");
    }

    #[test]
    fn format_zero() {
        assert_eq!(format(0.0), "0.00");
    }

    #[test]
    fn format_negative() {
        assert_eq!(format(-12.567), "-12.57");
    }

    #[test]
    fn to_dbfs_unity() {
        assert_eq!(to_dbfs(1.0), 0.0);
    }

    #[test]
    fn to_dbfs_half() {
        assert!((to_dbfs(0.5) - (-6.020_599_913_279_624)).abs() < 1e-10);
    }

    #[test]
    fn to_dbfs_tenth() {
        assert_eq!(to_dbfs(0.1), -20.0);
    }
}