use crate::{
    floating_point_math::floating_point_utils::map_sum_lossless,
    model::sonicprobe_error::SonicProbeError,
};

#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn compute_root_mean_square(values: &[f64]) -> Result<f64, SonicProbeError> {
    if values.is_empty() {
        return Err(SonicProbeError { 
            message: "input slice is empty".to_owned(), 
            location: format!("{}:{}", file!(), line!())
        })
    }

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

#[cfg(test)]
#[allow(clippy::float_cmp)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn single_positive_value() {
        let values = vec![4.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn single_negative_value() {
        let values = vec![-3.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn single_zero_value() {
        let values = vec![0.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn identical_positive_values() {
        let values = vec![2.0, 2.0, 2.0, 2.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn identical_negative_values() {
        let values = vec![-5.0, -5.0, -5.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn mixed_positive_negative() {
        let values = vec![3.0, -4.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 3.535_533_905_932_737_8);
    }

    #[test]
    fn zero_and_nonzero() {
        let values = vec![0.0, 6.0, 0.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert!((result - 3.464_101_615_137_754_4).abs() < 1e-10);
    }

    #[test]
    fn all_zeros() {
        let values = vec![0.0, 0.0, 0.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn empty_slice() {
        let values = vec![];
        let result = compute_root_mean_square(&values);
        assert!(result.is_err());
    }

    #[test]
    fn pythagorean_triple() {
        let values = vec![3.0, 4.0, 5.0];
        let result = compute_root_mean_square(&values).unwrap();
        assert_eq!(result, 4.082_482_904_638_63);
    }
}