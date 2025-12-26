pub mod analysis;
mod upscaler;

use crate::{
    dsp::{analysis::clipping::is_distorted, upscaler::Upscaler},
    model::{decibel::Decibel, frequency::Frequency, sonicprobe_error::SonicProbeError, Signal},
};

pub fn upsample_chain(
    source: &Signal,
    source_sample_rate: Frequency,
) -> Result<(Decibel, u64), SonicProbeError> {
    let mut upscaler = Upscaler::new(source, source_sample_rate)?;

    let mut peak = f64::MIN;
    let mut clipping_samples_count = 0u64;

    while let Some(sample) = upscaler.next_sample() {
        (peak, clipping_samples_count) = process_upsampled_sample(
            sample, &peak, &clipping_samples_count
        ) 
    }

    Ok((Decibel::new(peak), clipping_samples_count))
}

fn process_upsampled_sample(
    sample: &f64, peak: &f64, clipping_samples_count: &u64
) -> (f64, u64) {
    let abs_value = sample.abs();

    let new_clipping_count = match update_clipping_count(
        clipping_samples_count, &abs_value
    ) {
        Some(result) => result,
        None => *clipping_samples_count
    };

    let new_peak = match update_peak_value(peak, &abs_value){
        Some(result) => result,
        None => *peak
    };

    return (new_peak, new_clipping_count)
}

fn update_clipping_count(current_count: &u64, sample: &f64) -> Option<u64> {
    if is_distorted(*sample) {
        return Some(*current_count + 1)
    }

    None
}

fn update_peak_value(current_peak: &f64, sample: &f64) -> Option<f64> {
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