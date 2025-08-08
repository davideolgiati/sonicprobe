use crate::{
    audio_file::analysis::floating_point_utils::map_sum_lossless, audio_utils::to_dbfs,
    sonicprobe_error::SonicProbeError,
};

impl super::RootMeanSquare {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn process(values: &[f64]) -> Result<f64, SonicProbeError> {
        let sum = map_sum_lossless(values, |x| x.powi(2));

        let size = values.len() as f64;
        if (size as usize) != values.len() {
            return Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: format!(
                    "cannot represent usize value {} exactly in f64",
                    values.len()
                ),
            });
        }

        Ok(to_dbfs((sum / size).sqrt()))
    }
}
