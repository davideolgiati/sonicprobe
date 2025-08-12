
// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))

use crate::audio_file::Signal;


impl super::StereoCorrelation {
    #[inline]
    pub fn process(left: &Signal, right: &Signal) -> f64 {
        let mut left_square_sum: f64 = 0.0;
        let mut right_square_sum: f64 = 0.0;
        let mut correlation: f64 = 0.0;

        
        for i in 0..left.len() {
            correlation += left[i] * right[i];
            left_square_sum += left[i].powi(2);
            right_square_sum += right[i].powi(2);
        }

        correlation / (left_square_sum * right_square_sum).sqrt()
    }
}
