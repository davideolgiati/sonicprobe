// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))

use crate::model::Signal;

#[inline]
#[must_use] pub fn calculate_stereo_correlation(
    left: &Signal,
    right: &Signal
) -> f64 {
    let mut left_square_sum: f64 = 0.0;
    let mut right_square_sum: f64 = 0.0;
    let mut correlation: f64 = 0.0;

    for i in 0..left.len() {
        correlation += (left[i]) * (right[i]);
        left_square_sum += (left[i]).powi(2);
        right_square_sum += (right[i]).powi(2);
    }

    correlation / (left_square_sum * right_square_sum).sqrt()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn identical_signals() {
        let left = Arc::from(vec![1.0, 2.0, 3.0]);
        let right = Arc::from(vec![1.0, 2.0, 3.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn opposite_signals() {
        let left = Arc::from(vec![1.0, 2.0, 3.0]);
        let right = Arc::from(vec![-1.0, -2.0, -3.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!((result - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn zero_left_signal() {
        let left = Arc::from(vec![0.0, 0.0, 0.0]);
        let right = Arc::from(vec![1.0, 2.0, 3.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!(result.is_nan());
    }

    #[test]
    fn zero_right_signal() {
        let left = Arc::from(vec![1.0, 2.0, 3.0]);
        let right = Arc::from(vec![0.0, 0.0, 0.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!(result.is_nan());
    }

    #[test]
    fn orthogonal_signals() {
        let left = Arc::from(vec![1.0, 0.0, -1.0]);
        let right = Arc::from(vec![0.0, 1.0, 0.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn partial_correlation() {
        let left = Arc::from(vec![1.0, 2.0, 0.0]);
        let right = Arc::from(vec![2.0, 4.0, 3.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!((result - 0.830_454_798_537_399_7).abs() < 1e-10);
    }

    #[test]
    fn single_element() {
        let left = Arc::from(vec![5.0]);
        let right = Arc::from(vec![3.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn negative_correlation() {
        let left = Arc::from(vec![1.0, 3.0, 2.0]);
        let right = Arc::from(vec![-2.0, -1.0, -3.0]);
        let result = calculate_stereo_correlation(&left, &right);
        assert!(result < 0.0);
    }
}