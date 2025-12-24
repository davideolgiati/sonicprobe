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
    let mut upscaler = Upscaler::new(source_sample_rate)?;

    let mut peak = f64::MIN;
    let mut clipping_samples_count = 0u64;

    for i in 0..source.len() - 12 {
        for sample in upscaler.upscale(&source[i..i + 12]) {
            (peak, clipping_samples_count) = process_upsampled_sample(
                sample, &peak, &clipping_samples_count
            ) 
        }
    }

    Ok((Decibel::new(peak), clipping_samples_count))
}

pub fn process_upsampled_sample(
    sample: &f64, peak: &f64, clipping_samples_count: &u64
) -> (f64, u64) {
    let abs_value = sample.abs();

    let new_clipping_samples_count = {
        if is_distorted(abs_value) {
            clipping_samples_count + 1
        } else {
            *clipping_samples_count
        }
    };

    let new_peak = {
        if abs_value > *peak {
            abs_value
        } else {
            *peak
        }
    };

    return (new_peak, new_clipping_samples_count)
}