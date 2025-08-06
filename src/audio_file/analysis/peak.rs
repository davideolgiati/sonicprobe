use std::cmp::Ordering;

use crate::{audio_file::Signal, audio_utils::to_dbfs};

impl super::Peak {
    #[inline]
    pub fn process(samples: &Signal) -> f64 {
        match samples
            .iter()
            .max_by(|&item1, &item2| item1.partial_cmp(item2).unwrap_or(Ordering::Equal))
        {
            Some(&value) => to_dbfs(value),
            None => to_dbfs(0.0),
        }
    }
}
