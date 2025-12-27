pub mod analysis;
mod upscaler;

use crate::{
    dsp::{analysis::clipping::update_clipping_count, upscaler::Upscaler},
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
        match update_clipping_count(&clipping_samples_count, sample) {
            Some(result) => clipping_samples_count = result,
            None => {}
        }

        match update_peak_value(&peak, &sample.abs()) {
            Some(result) => peak = result,
            None => {}
        }
    }

    Ok((Decibel::new(peak), clipping_samples_count))
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