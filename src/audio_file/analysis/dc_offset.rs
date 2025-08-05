use crate::audio_file::analysis::floating_point_utils::map_sum_lossless;

impl super::DCOffset {
    #[inline]
    pub fn process(values: &[f64]) -> f64 {
        let sum = map_sum_lossless(values, |x| x);
        sum / values.len() as f64
    }
}
