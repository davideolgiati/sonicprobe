impl super::RootMeanSquare {
    pub fn new() -> super::RootMeanSquare {
        super::RootMeanSquare {
            partials: Vec::new(),
            count: 0,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        let mut current = (value as f64).powi(2);
        let mut index: usize = 0;

        //TODO: da portare fuori
        for mut partial in self.partials.clone() {
            if current.abs() < partial.abs() {
                (current, partial) = (partial, current)
            }

            let high = current + partial;
            let low = partial - (high - current);

            if low != 0.0 {
                self.partials[index] = low;
                index += 1;
            }
            current = high
        }

        self.partials.truncate(index);
        self.partials.push(current);
        self.count += 1;
    }

    pub fn build(&self) -> f32 {
        if self.count == 0 {
            return 0.0f32;
        }

        let sum: f64 = self.partials.iter().sum();

        (sum / self.count as f64).sqrt() as f32
    }
}

/*
#[cfg(test)]
mod tests {
    // genarati dall' AI ma rivisti da me per valuarne la correttezza
    use super::*;

    #[test]
    fn test_new_creates_empty_builder() {
        let builder = RMSBuilder::new();
        assert_eq!(builder.count, 0);
        assert_eq!(builder.partials.len(), 0);
    }

    #[test]
    fn test_add_single_value() {
        let mut builder = RMSBuilder::new();
        builder.add(3.0);
        let result = builder.build();
        assert_eq!(result, 3.0); // RMS of single value is the value itself
    }

    #[test]
    fn test_add_multiple_values() {
        let mut builder = RMSBuilder::new();
        builder.add(3.0);
        builder.add(4.0);
        builder.add(5.0);
        let result = builder.build();
        // RMS of [3, 4, 5] = sqrt((9 + 16 + 25) / 3) = sqrt(50/3) ≈ 4.08
        assert!((result - 4.08248).abs() < 0.001);
    }

    #[test]
    fn test_zero_values() {
        let mut builder = RMSBuilder::new();
        builder.add(0.0);
        builder.add(0.0);
        let result = builder.build();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_negative_values() {
        let mut builder = RMSBuilder::new();
        builder.add(-3.0);
        builder.add(-4.0);
        let result = builder.build();
        // RMS of [-3, -4] = sqrt((9 + 16) / 2) = sqrt(12.5) = 3.536
        assert!((result - 3.5355).abs() < 0.001);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let mut builder = RMSBuilder::new();
        builder.add(-3.0);
        builder.add(4.0);
        let result = builder.build();
        // RMS of [-3, 4] = sqrt((9 + 16) / 2) = sqrt(12.5) = 3.536
        assert!((result - 3.5355).abs() < 0.001);
    }

    #[test]
    fn test_build_without_adding_values() {
        let builder = RMSBuilder::new();
        let result = builder.build();
        // This tests edge case - might return 0.0 or NaN depending on implementation
        assert!(result.is_nan() || result == 0.0);
    }

    #[test]
    fn test_multiple_builds() {
        let mut builder = RMSBuilder::new();
        builder.add(3.0);
        builder.add(4.0);

        let result1 = builder.build();
        let result2 = builder.build();

        // Build should be idempotent
        assert_eq!(result1, result2);
    }
}
*/
