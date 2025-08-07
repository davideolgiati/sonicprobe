use std::{sync::Arc, thread};

use crate::{
    audio_file::{
        analysis::{ClippingSamples, Peak},
        types::{Frequency, Signal},
    },
    constants::LOW_PASS_FILTER_SIZE,
    sonicprobe_error::SonicProbeError,
};

mod low_pass_filter;
mod upsampler;

pub struct LowPassFilter {
    coeffs: Arc<[f64]>,
}

struct Upsampler {
    multipier: u8,
}

pub fn upsample(source: Signal, sample_rate: Frequency) -> std::thread::JoinHandle<Result<(f64, usize), SonicProbeError>> {
    thread::spawn(move || {
        let signal = upsample_chain(&source, sample_rate)?;

        let peak = Peak::process(&signal);
        let clip_count = ClippingSamples::process(&signal);

        Ok((peak, clip_count))
    })
}

fn upsample_chain(source: &Signal, source_sample_rate: u32) -> Result<Signal, SonicProbeError> {
    let upsampler = Upsampler::new(source_sample_rate)?;
    let low_pass = LowPassFilter::new(source_sample_rate)?;

    let upsampled_signal: Vec<f64> = (0..(source.len() - 4))
        .flat_map(|start| upsampler.submit(source, start))
        .flatten()
        .collect();

    Ok(upsampled_signal
        .windows(LOW_PASS_FILTER_SIZE)
        .map(|window| low_pass.submit(window))
        .collect())
}
