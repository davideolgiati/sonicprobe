
// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))

use crate::audio_file::Signal;


impl super::StereoCorrelation {
    #[inline]
    pub fn process(left: &Signal, right: &Signal) -> f64 {
        let correlation: f64 = left
            .iter()
            .zip(right.iter())
            .map(|(x, y)| x * y)
            .sum();
        let left_square_sum: f64 = left.iter().map(|x| x.powi(2)).sum();
        let right_square_sum: f64 = right.iter().map(|x| x.powi(2)).sum();

        correlation / (left_square_sum * right_square_sum).sqrt()
    }
}
