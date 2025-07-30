use rayon::prelude::*;

use crate::builders::TrueBitDepthBuilder;

impl TrueBitDepthBuilder {
    pub fn new(depth: u8, sample_count: u64) -> TrueBitDepthBuilder {
        TrueBitDepthBuilder {
            min: u8::MAX,
            max: u8::MIN,
            avarage: 0.0,
            reported_depth: depth,
            sample_count,
        }
    }

    #[inline]
    pub fn add(&mut self, mapped_stream: Vec<f32>, factor: f32) {
        let mut real_depths = mapped_stream
            .par_iter()
            .map(|sample| {
                if *sample == 0.0 {
                    return 0u8;
                }

                let trailing_zeros = ((*sample * factor) as i32).trailing_zeros();
                (self.reported_depth as u32 - trailing_zeros) as u8
            })
            .collect::<Vec<u8>>();

        real_depths.sort();

        self.min = real_depths[0];
        self.max = real_depths[real_depths.len() - 1];
        self.avarage = (real_depths.par_iter().map(|s| *s as u64).sum::<u64>() as f64
            / (self.sample_count * 2) as f64) as f32
    }

    pub fn build(&self) -> (u8, u8, u8) {
        (self.min, self.max, self.avarage.round() as u8)
    }
}
