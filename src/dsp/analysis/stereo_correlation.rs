// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))

use crate::model::Signal;

impl super::StereoCorrelation {
    #[inline]
    pub fn process(left: &Signal, right: &Signal, left_offset: f64, right_offset: f64) -> f64 {
        let mut left_square_sum: f64 = 0.0;
        let mut right_square_sum: f64 = 0.0;
        let mut correlation: f64 = 0.0;

        
        for i in 0..left.len() {
            correlation += (left[i] - left_offset) * (right[i] - right_offset);
            left_square_sum += (left[i] - left_offset).powi(2);
            right_square_sum += (right[i] - right_offset).powi(2);
        }

        correlation / (left_square_sum * right_square_sum).sqrt()
    }
}
