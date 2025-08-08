use std::sync::Arc;

use crate::{
    audio_file::types::Signal,
    sonicprobe_error::SonicProbeError,
};

mod low_pass_filter;

pub struct LowPassFilter {
    coeffs: Arc<[[f64; 12]]>,
}

pub fn upsample_chain(source: &Signal, source_sample_rate: u32) -> Result<Signal, SonicProbeError> {
    let low_pass = LowPassFilter::new(source_sample_rate)?;

    Ok(source
        .windows(12)
        .flat_map(|window| low_pass.submit(window))
        .collect())
}
