pub fn update_peak_value(current_peak: &f64, sample: &f64) -> Option<f64> {
    if *sample > *current_peak {
        return Some(*sample)
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn update_peak_value_change() {
        let mut rng = rand::rng();
        let current_peak = rng.random_range(0.0..0.5);
        let sample = rng.random_range(0.50000001..1.0);

        let result = update_peak_value(&current_peak, &sample);

        assert_eq!(result, Some(sample))
    }

    #[test]
    fn update_peak_value_no_change() {
        let mut rng = rand::rng();
        let current_peak = rng.random_range(0.0..0.5);
        let sample = rng.random_range(-1.0..current_peak);

        let result = update_peak_value(&current_peak, &sample);

        assert_eq!(result, None)
    }
}