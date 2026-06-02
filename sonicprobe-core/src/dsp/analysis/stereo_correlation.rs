// norm_corr_ab = sum(a*b) / sqrt(sum(a^2)*sum(b^2))

use crate::model::Signal;

/// Independent accumulator lanes per sum, to break the serial `addsd` chains.
///
/// Lower than the RMS hot path's 8 lanes: this loop carries THREE sums, so
/// `CORR_LANES * 3` accumulators compete for the 16 xmm registers — too many
/// lanes spill to the stack and lose more than the extra ILP buys.
const CORR_LANES: usize = 2;

#[inline]
pub fn calculate_stereo_correlation(
    left: &Signal,
    right: &Signal
) -> f64 {
    let mut correlation_lanes = [0.0f64; CORR_LANES];
    let mut left_square_lanes = [0.0f64; CORR_LANES];
    let mut right_square_lanes = [0.0f64; CORR_LANES];

    // Fixed-width `[f64; CORR_LANES]` chunks: the inner index carries no bounds
    // check and the lane loop unrolls into independent dependency chains.
    let (left_chunks, left_remainder) = left.as_chunks::<CORR_LANES>();
    let (right_chunks, right_remainder) = right.as_chunks::<CORR_LANES>();

    for (left_chunk, right_chunk) in left_chunks.iter().zip(right_chunks.iter()) {
        for lane in 0..CORR_LANES {
            correlation_lanes[lane] += left_chunk[lane] * right_chunk[lane];
            left_square_lanes[lane] += left_chunk[lane] * left_chunk[lane];
            right_square_lanes[lane] += right_chunk[lane] * right_chunk[lane];
        }
    }

    let mut correlation: f64 = 0.0;
    let mut left_square_sum: f64 = 0.0;
    let mut right_square_sum: f64 = 0.0;
    for lane in 0..CORR_LANES {
        correlation += correlation_lanes[lane];
        left_square_sum += left_square_lanes[lane];
        right_square_sum += right_square_lanes[lane];
    }

    // Tail samples that didn't fill a full lane-width chunk.
    let remainder = left_remainder.len().min(right_remainder.len());
    for i in 0..remainder {
        correlation += left_remainder[i] * right_remainder[i];
        left_square_sum += left_remainder[i] * left_remainder[i];
        right_square_sum += right_remainder[i] * right_remainder[i];
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