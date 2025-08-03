use std::{cmp::Ordering, sync::Arc};

impl super::PeakBuilder {
    #[inline]
    pub fn process(samples: &Arc<[f32]>) -> f32 {
        match samples
            .iter()
            .enumerate()
            .max_by(|&(_, item1), &(_, item2)| item1.partial_cmp(item2).unwrap_or(Ordering::Equal))
        {
            Some((_, &value)) => value,
            None => f32::MIN,
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single_value() {
        let result = PeakBuilder::process(&[3.0]);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_add_multiple_values() {
        let result = PeakBuilder::process(&[3.0, 4.0, 5.0]);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_zero_values() {
        let result = PeakBuilder::process(&[0.0, 0.0]);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_negative_values() {
        let result = PeakBuilder::process(&[-3.0, -4.0]);
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let result = PeakBuilder::process(&[-3.0, 4.0]);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_order_independence() {
        let res1 = PeakBuilder::process(&[1.0, 5.0, 3.0]);
        let res2 = PeakBuilder::process(&[3.0, 1.0, 5.0]);

        assert_eq!(res1, res2);
    }

    #[test]
    fn test_extreme_values() {
        let result = PeakBuilder::process(&[f32::MAX, f32::MIN, 0.0]);
        assert_eq!(result, f32::MAX);
    }

    #[test]
    fn test_infinity_values() {
        let result = PeakBuilder::process(&[f32::INFINITY, 100.0]);
        assert_eq!(result, f32::INFINITY);
    }

    #[test]
    fn test_nan_values() {
        let result = PeakBuilder::process(&[f32::NAN, 5.0]);
        assert!(result.is_nan() || result == 5.0);
    }
}
*/