use crate::builders::PeakBuilder;

impl PeakBuilder {
    pub fn new() -> PeakBuilder {
        PeakBuilder {
            current_max: f32::MIN,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        if value > self.current_max {
            self.current_max = value
        }
    }

    pub fn build(self) -> f32 {
        self.current_max
    }
}

#[cfg(test)]
mod tests {
    // genarati dall' AI ma rivisti da me per valuarne la correttezza
    use super::*;

    #[test]
    fn test_new_creates_empty_builder() {
        let builder = PeakBuilder::new();
        assert_eq!(builder.current_max, f32::MIN);
    }

    #[test]
    fn test_add_single_value() {
        let mut builder = PeakBuilder::new();
        builder.add(3.0);
        let result = builder.build();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_add_multiple_values() {
        let mut builder = PeakBuilder::new();
        builder.add(3.0);
        builder.add(4.0);
        builder.add(5.0);
        let result = builder.build();
        assert_eq!(result, 5.0); // Peak is the maximum value
    }

    #[test]
    fn test_zero_values() {
        let mut builder = PeakBuilder::new();
        builder.add(0.0);
        builder.add(0.0);
        let result = builder.build();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_negative_values() {
        let mut builder = PeakBuilder::new();
        builder.add(-3.0);
        builder.add(-4.0);
        let result = builder.build();
        assert_eq!(result, -3.0); // Maximum of negative values
    }

    #[test]
    fn test_mixed_positive_negative() {
        let mut builder = PeakBuilder::new();
        builder.add(-3.0);
        builder.add(4.0);
        let result = builder.build();
        assert_eq!(result, 4.0); // Positive is greater than negative
    }

    #[test]
    fn test_build_without_adding_values() {
        let builder = PeakBuilder::new();
        let result = builder.build();
        assert_eq!(result, f32::MIN); // Returns initial value if no values added
    }

    #[test]
    fn test_order_independence() {
        let mut builder1 = PeakBuilder::new();
        let mut builder2 = PeakBuilder::new();

        // Add same values in different order
        builder1.add(1.0);
        builder1.add(5.0);
        builder1.add(3.0);

        builder2.add(3.0);
        builder2.add(1.0);
        builder2.add(5.0);

        assert_eq!(builder1.build(), builder2.build());
    }

    #[test]
    fn test_extreme_values() {
        let mut builder = PeakBuilder::new();
        builder.add(f32::MAX);
        builder.add(f32::MIN);
        builder.add(0.0);
        let result = builder.build();
        assert_eq!(result, f32::MAX);
    }

    #[test]
    fn test_infinity_values() {
        let mut builder = PeakBuilder::new();
        builder.add(f32::INFINITY);
        builder.add(100.0);
        let result = builder.build();
        assert_eq!(result, f32::INFINITY);
    }

    #[test]
    fn test_nan_values() {
        let mut builder = PeakBuilder::new();
        builder.add(f32::NAN);
        builder.add(5.0);
        let result = builder.build();
        // NaN comparisons are always false, so 5.0 should be the max
        // (implementation dependent - might need adjustment)
        assert!(result.is_nan() || result == 5.0);
    }
}
