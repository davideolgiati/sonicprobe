pub mod analysis;
mod low_pass_filter;

use crate::{
    dsp::{analysis::clipping::is_distorted, low_pass_filter::LowPassFilter},
    model::{decibel::Decibel, frequency::Frequency, sonicprobe_error::SonicProbeError, Signal},
};

pub fn upsample_chain(
    source: &Signal,
    source_sample_rate: Frequency,
) -> Result<(Decibel, u64), SonicProbeError> {
    let low_pass = LowPassFilter::new(source_sample_rate)?;

    let mut peak_h = f64::MIN;
    let mut peak_l = f64::MAX;
    let mut clipping_samples = 0u64;

    for i in 0..source.len() - 12 {
        for value in low_pass.submit(&source[i..i + 12]) {
            if is_distorted(value) {
                clipping_samples += 1;
            }

            if value > peak_h {
                peak_h = value;
            } else if value < peak_l {
                peak_l = value;
            } 
        }
    }

    let peak = {
        if peak_l.abs() > peak_h {
            peak_l.abs()
        } else {
            peak_h
        }
    };

    Ok((Decibel::new(peak), clipping_samples))
}
