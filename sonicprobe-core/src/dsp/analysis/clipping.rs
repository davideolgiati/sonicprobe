pub fn update_clipping_count(current_count: &u64, sample: &f64) -> Option<u64> {
    if is_distorted(*sample) {
        return Some(*current_count + 1)
    }

    None
}

const fn is_distorted(sample: f64) -> bool {
    sample >= 1.0 || sample <= -1.0
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn update_clipping_count_increase() {
        let mut rng = rand::rng();
        let clipping_sample: f64 = rng.random_range(1.0..2.0);
        let current_clipping_sample_count = 0;

        let result = update_clipping_count(
            &current_clipping_sample_count, 
            &clipping_sample
        ); 

        assert_eq!(result, Some(1));
    }

    #[test]
    fn update_clipping_count_no_increase() {
        let mut rng = rand::rng();
        let sample: f64 = rng.random_range(-0.9..0.9);
        let current_clipping_sample_count = 0;

        let result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, None);
    }

    #[test]
    fn update_clipping_count_edge_cases() {
        let mut sample: f64 = 1.0;
        let current_clipping_sample_count = 0;

        let mut result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, Some(1));

        sample = -1.0;
        result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, Some(1));

        sample = 0.9999999;
        result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, None);

        sample = -0.9999999;
        result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, None);

        sample = 1.0000001;
        result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, Some(1));

        sample = -1.0000001;
        result = update_clipping_count(
            &current_clipping_sample_count, 
            &sample
        ); 

        assert_eq!(result, Some(1));
    }

    #[test]
    fn is_distorted_truthy() {
        let mut rng = rand::rng();
        
        let result_positive = {
            let clipping_sample: f64 = rng.random_range(1.0..2.0);
            is_distorted(clipping_sample)
        };

        assert_eq!(result_positive, true);

        let result_negative = {
            let clipping_sample: f64 = rng.random_range(-2.0..-1.0);
            is_distorted(clipping_sample)
        };

        assert_eq!(result_negative, true)
    }

        #[test]
    fn is_distorted_falsy() {
        let mut rng = rand::rng();
        let clipping_sample: f64 = rng.random_range(-0.9999999..0.9999999);

        let result = is_distorted(clipping_sample); 

        assert_eq!(result, false);
    }

        #[test]
    fn is_distorted_edges() {
        let result_upper = is_distorted(1.0); 

        assert_eq!(result_upper, true);

        let result_lower = is_distorted(-1.0); 

        assert_eq!(result_lower, true);
    }

}
