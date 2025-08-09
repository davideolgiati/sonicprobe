use std::sync::Arc;

use crate::{
    audio_file::{analysis::clipping_samples::is_clipping, types::Signal},
    sonicprobe_error::SonicProbeError,
};

mod low_pass_filter;

pub struct LowPassFilter {
    coeffs: Arc<[[f64; 12]]>,
}

pub fn upsample_chain(source: &Signal, source_sample_rate: u32) -> Result<(f64, u64), SonicProbeError> {
    let low_pass = LowPassFilter::new(source_sample_rate)?;

    let mut peak = 0.0;
    let mut clipping_samples = 0u64;

    for i in 0..source.len() - 12 {
        for value in low_pass.submit(&source[i..i+12]) {
            if is_clipping(value) {
                clipping_samples += 1
            }

            if value.abs() > peak {
                peak = value
            }
        }
    }

    Ok((peak, clipping_samples))
}
