use crate::{audio_file::analysis::floating_point_utils::map_sum_lossless, audio_utils::to_dbfs};

impl super::RootMeanSquare {
    #[inline]
    pub fn process(values: &[f64]) -> f64 {
        let sum = map_sum_lossless(values, |x| x.powi(2));
        to_dbfs((sum / values.len() as f64).sqrt())
    }
}