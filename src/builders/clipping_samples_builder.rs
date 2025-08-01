use std::sync::Arc;

use crate::builders::ClippingSamplesBuilder;

impl ClippingSamplesBuilder {
    #[inline]
    pub fn process(samples: &Arc<[f32]>) -> usize {
        samples
            .iter()
            .filter(|&&x| is_clipping(x))
            .map(|_| 1)
            .sum()
    }
}

pub fn is_clipping(sample: f32) -> bool {
    (sample.abs() - 0.95).abs() < f32::EPSILON || sample.abs() > 0.95
}
