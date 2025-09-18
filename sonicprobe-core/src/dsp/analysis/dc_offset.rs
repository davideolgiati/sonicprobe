use crate::{
    floating_point_math::floating_point_utils::map_sum_lossless,
    model::{sonicprobe_error::SonicProbeError, Signal},
};

/// Given a `mono_signal`, loops over each sample and compute the avarage of the
/// input signal values.
/// 
/// Returns `Ok(samples_sum / total_samples_count)` an f64 varaible containing 
/// the avarage of all samples in the signal on success, 
/// otherwise returns an error.
/// 
/// This function has no side effects.
/// This function is declared as `#[inline]`
///
/// # Errors
/// 
/// Returns [`SonicProbeError`](crate::model::sonicprobe_error::SonicProbeError) 
/// if casting the signal length from usize to f64 fails
///
/// # Examples
/// 
/// ```
///     let mut rng = rand::rng();
///     let samples = (0..10).map(|_| rng.random_range(-0.99..0.99)).collect();
///     let res = count_clipping_samples(&samples)
/// ```
///
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn calculate_dc_offset(mono_signal: &Signal) -> Result<f64, SonicProbeError> {
    let samples_sum = map_sum_lossless(mono_signal, |x| x);
    let total_samples_count = mono_signal.len() as f64;

    if (total_samples_count as usize) != mono_signal.len() {
        return Err(SonicProbeError {
            location: format!("{}:{}", file!(), line!()),
            message: format!(
                "cannot represent usize value {} exactly in f64",
                mono_signal.len()
            ),
        });
    }

    Ok(samples_sum / total_samples_count)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::Arc;

    use crate::dsp::analysis::dc_offset::calculate_dc_offset;

    #[test]
    fn zero() {
        let samples: Arc<[f64]> = (1..11)
            .map(|i| {
                if i % 2 == 0 {
                    f64::from(-(i / 2))
                } else {
                    f64::from((i + 1) / 2)
                }
            })
            .collect();
        let res = calculate_dc_offset(&samples).unwrap();

        assert!((res - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn positive() {
        let samples: Arc<[f64]> = (1..11)
            .map(|i| {
                if i % 2 == 0 {
                    f64::from(-(i / 2))
                } else {
                    f64::from((i + 1) / 2)
                }
            })
            .map(|val| val + 0.002)
            .collect();
        let res = calculate_dc_offset(&samples).unwrap();

        assert!((res - 0.002).abs() < f64::EPSILON);
    }

    #[test]
    fn negative() {
        let samples: Arc<[f64]> = (1..11)
            .map(|i| {
                if i % 2 == 0 {
                    f64::from(-(i / 2))
                } else {
                    f64::from((i + 1) / 2)
                }
            })
            .map(|val| val - 0.002)
            .collect();
        let res = calculate_dc_offset(&samples).unwrap();

        assert!((res + 0.002).abs() < f64::EPSILON);
    }
}
