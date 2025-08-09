
// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))

use crate::audio_file::Signal;


impl super::StereoCorrelation {
    #[inline]
    pub fn process(source: &Signal) -> f64 {
        let mut left_square_sum: f64 = 0.0;
        let mut right_square_sum: f64 = 0.0;
        let mut correlation: f64 = 0.0;

        
        for chunk in source.chunks(2) {
            correlation += chunk[0] * chunk[1];
            left_square_sum += chunk[0].powi(2);
            right_square_sum += chunk[1].powi(2);
        }

        correlation / (left_square_sum * right_square_sum).sqrt()
    }
}
