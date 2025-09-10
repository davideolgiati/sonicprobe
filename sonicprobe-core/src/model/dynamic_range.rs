use serde::Serialize;

use crate::model::decibel::Decibel;

#[derive(Clone, Copy, Serialize)]
pub struct DynamicRange(i64);

impl From<Decibel> for DynamicRange {
    #[allow(clippy::cast_possible_truncation)]
    fn from(obj: Decibel) -> Self {
        let value = obj.get_value().round().abs() as i64;
        Self(value)
    }
}

impl DynamicRange {
        #[must_use] pub fn get_string_value(self) -> String {
                format!("{}", self.0)
        }

        #[must_use] pub fn get_unit() -> String {
                "DR".to_owned()
        }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn from_positive_decibel() {
        let db = Decibel::new(2.0);
        let dr: DynamicRange = db.into();
        assert_eq!(dr.get_string_value(), "6");
    }

    #[test]
    fn from_zero_decibel() {
        let db = Decibel::new(1.0);
        let dr: DynamicRange = db.into();
        assert_eq!(dr.get_string_value(), "0");
    }

    #[test]
    fn from_negative_decibel() {
        let db = Decibel::new(0.1);
        let dr: DynamicRange = db.into();
        assert_eq!(dr.get_string_value(), "20");
    }

    #[test]
    fn from_decibel_rounds() {
        let db = Decibel::new(0.316_227_766_016_837_94);
        let dr: DynamicRange = db.into();
        assert_eq!(dr.get_string_value(), "10");
    }

    #[test]
    fn get_string_value() {
        let dr = DynamicRange(42);
        assert_eq!(dr.get_string_value(), "42");
    }

    #[test]
    fn get_unit() {
        assert_eq!(DynamicRange::get_unit(), "DR");
    }

    #[test]
    fn serialize() {
        let dr = DynamicRange(15);
        let json = serde_json::to_string(&dr).unwrap();
        assert_eq!(json, "15");
    }
}