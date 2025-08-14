use crate::{
    audio_file::analysis::floating_point_utils::map_sum_lossless, sonicprobe_error::SonicProbeError,
};

impl super::DCOffset {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn process(values: &[f64]) -> Result<f64, SonicProbeError> {
        let sum = map_sum_lossless(values, |x| x);

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

        Ok(sum / size)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::sync::Arc;

    use crate::audio_file::analysis::DCOffset;


    #[test]
    fn zero() {
        let samples: Arc<[f64]> = (1..11).map(|i| {
            if i % 2 == 0 {
                f64::from(-(i / 2))
            } else {
                f64::from((i + 1) / 2)
            }
        })
        .collect();
        let res = DCOffset::process(&samples).unwrap();

        assert!((res - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn positive() {
        let samples: Arc<[f64]> = (1..11).map(|i| {
            if i % 2 == 0 {
                f64::from(-(i / 2))
            } else {
                f64::from((i + 1) / 2)
            }
        })
        .map(|val| val + 0.002)
        .collect();
        let res = DCOffset::process(&samples).unwrap();

        assert!((res - 0.002).abs() < f64::EPSILON);
    }

    #[test]
    fn negative() {
                let samples: Arc<[f64]> = (1..11).map(|i| {
            if i % 2 == 0 {
                f64::from(-(i / 2))
            } else {
                f64::from((i + 1) / 2)
            }
        })
        .map(|val| val - 0.002)
        .collect();
        let res = DCOffset::process(&samples).unwrap();

        assert!((res + 0.002).abs() < f64::EPSILON);
    }
}