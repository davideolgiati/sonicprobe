pub mod analysis;
mod upscaler;

use crate::{
    analysis::peak::update_peak_value, dsp::{analysis::clipping::update_clipping_count, upscaler::Upscaler}, model::{Signal, decibel::Decibel, frequency::Frequency, sonicprobe_error::SonicProbeError}
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
