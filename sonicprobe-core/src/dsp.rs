pub mod analysis;
mod upscaler;

use crate::{
    dsp::upscaler::upsample_reduce,
    model::{Signal, decibel::Decibel, frequency::Frequency, sonicprobe_error::SonicProbeError},
};

pub fn upsample_chain(
    source: &Signal,
    source_sample_rate: Frequency,
) -> Result<(Decibel, u64), SonicProbeError> {
    let (peak, clipping_samples_count) = upsample_reduce(source, source_sample_rate)?;

    Ok((Decibel::new(peak), clipping_samples_count))
}
