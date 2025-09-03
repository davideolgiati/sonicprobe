use std::ops::Sub;

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
        pub fn get_string_value(self) -> String {
                format!("{}", self.0)
        }

        pub fn get_unit() -> String {
                "DR".to_owned()
        }
}

impl Sub for DynamicRange {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}