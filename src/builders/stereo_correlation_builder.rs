use std::sync::Arc;

// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))
use rayon::prelude::*;

use crate::builders::StereoCorrelationBuilder;

impl StereoCorrelationBuilder {
    #[inline]
    pub fn process(left: &Arc<[f32]>, right: &Arc<[f32]>) -> f32 {
        let correlation: f32 = left
            .par_iter()
            .zip(right.par_iter())
            .map(|(x, y)| x * y)
            .sum();
        let left_square_sum: f32 = left.par_iter().map(|x| x.powi(2)).sum();
        let right_square_sum: f32 = right.par_iter().map(|x| x.powi(2)).sum();

        correlation / (left_square_sum * right_square_sum).sqrt()
    }
}
