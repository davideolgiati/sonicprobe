// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))
use rayon::prelude::*;

use crate::builders::StereoCorrelationBuilder;

impl StereoCorrelationBuilder {
        pub fn new() -> StereoCorrelationBuilder {
                StereoCorrelationBuilder {
                        correlation: 0.0,
                        left_square_sum: 0.0,
                        right_square_sum: 0.0
                }
        }

        #[inline]
        pub fn add(&mut self, left: &[f32], right: &[f32]) {
                self.correlation = left.par_iter().zip(right.par_iter()).map(|(x, y)| x*y).sum();
                self.left_square_sum = left.par_iter().map(|x| x.powi(2)).sum();
                self.right_square_sum = right.par_iter().map(|x| x.powi(2)).sum();
        }

        pub fn build(&self) -> f32 {
               self.correlation / (self.left_square_sum * self.right_square_sum).sqrt()
        }
}