use std::sync::Arc;

use rayon::prelude::*;

use crate::builders::TrueBitDepthBuilder;

impl TrueBitDepthBuilder {
    #[inline]
    pub fn process(signal: Arc<[f32]>, factor: f32, reported_depth: u8) -> u8 {
        signal
            .par_iter()
            .map(|sample| {
                if *sample == 0.0 {
                    return 0u8;
                }

                let trailing_zeros = ((*sample * factor) as i32).trailing_zeros();
                (reported_depth as u32 - trailing_zeros) as u8
            })
            .max_by(|a, b| a.cmp(b))
            .unwrap()
    }
}
