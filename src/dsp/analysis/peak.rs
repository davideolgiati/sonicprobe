use std::cmp::Ordering;

use crate::{audio_utils::to_dbfs, model::Signal};

#[inline]
pub fn find_signal_peak(samples: &Signal) -> f64 {
    samples
        .iter()
        .map(|x| x.abs())
        .max_by(|&item1, &item2| item1.partial_cmp(&item2).unwrap_or(Ordering::Equal))
        .map_or_else(|| to_dbfs(0.0), to_dbfs)
}
