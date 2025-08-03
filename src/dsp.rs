use std::sync::Arc;

use crate::{audio_file::Signal, constants::LOW_PASS_FILTER_SIZE};

mod dsp_chain;
mod low_pass_filter;
mod upsampler;



pub struct LowPassFilter {
    coeffs: [f64; LOW_PASS_FILTER_SIZE],
}

struct Upsampler {
    multipier: u8,
}

struct DSPChain<T> {
    data: Arc<[T]>,
}

pub fn upsample(samples: Signal, original_sample_rate: u32) -> Signal {
    let upsampler = Upsampler::new(original_sample_rate);
    let low_pass = LowPassFilter::new(original_sample_rate);

    DSPChain::new(samples)
        .flat_window(4, |window: Signal, start: usize| {
            upsampler.submit(window, start)
        })
        .window(
            crate::dsp::LOW_PASS_FILTER_SIZE,
            |window: &Signal, start: usize, end: usize| low_pass.submit(window, start, end),
        )
        .collect()
}
