use std::sync::Arc;

use crate::constants::LOW_PASS_FILTER_SIZE;

mod dsp_chain;
mod low_pass_filter;
mod upsampler;



pub struct LowPassFilter {
    coeffs: [f32; LOW_PASS_FILTER_SIZE],
}

struct Upsampler {
    multipier: u8,
}

struct DSPChain<T> {
    data: Arc<[T]>,
}

pub fn upsample(samples: Arc<[f32]>, original_sample_rate: u32) -> Arc<[f32]> {
    let upsampler = Upsampler::new(original_sample_rate);
    let low_pass = LowPassFilter::new(original_sample_rate);

    DSPChain::new(samples)
        .flat_window(4, |window: Arc<[f32]>, start: usize, _end: usize| {
            upsampler.submit(window, start, _end)
        })
        .window(
            crate::dsp::LOW_PASS_FILTER_SIZE,
            |window: &Arc<[f32]>, start: usize, end: usize| low_pass.submit(window, start, end),
        )
        .collect()
}
