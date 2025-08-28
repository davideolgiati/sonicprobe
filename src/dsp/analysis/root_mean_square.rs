use crate::{
    floating_point_math::floating_point_utils::map_sum_lossless,
    model::sonicprobe_error::SonicProbeError,
};

#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn compute_root_mean_square(values: &[f64]) -> Result<f64, SonicProbeError> {
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

    Ok((sum / size).sqrt())
}
