pub mod analysis;
mod low_pass_filter;

use crate::{
    audio_utils::to_dbfs,
    dsp::{analysis::clipping::is_distorted, low_pass_filter::LowPassFilter},
    model::{frequency::Frequency, sonicprobe_error::SonicProbeError, Signal},
};

pub fn upsample_chain(
    source: &Signal,
    source_sample_rate: Frequency,
) -> Result<(f64, u64), SonicProbeError> {
    let low_pass = LowPassFilter::new(source_sample_rate)?;

    let mut peak = f64::MIN;
    let mut clipping_samples = 0u64;

    for i in 0..source.len() - 12 {
        for value in low_pass.submit(&source[i..i + 12]) {
            if is_distorted(value) {
                clipping_samples += 1;
            }

            if value.abs() > peak {
                peak = value.abs();
            }
        }
    }

    Ok((to_dbfs(peak), clipping_samples))
}
