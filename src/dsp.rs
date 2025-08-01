use std::sync::Arc;

mod dsp_chain;
mod low_pass_filter;
mod upsampler;

const TARGET_FREQUENCY: u32 = 192_000;
pub const LOW_PASS_FILTER_SIZE: usize = 48;

pub struct LowPassFilter {
    coeffs: [f32; LOW_PASS_FILTER_SIZE],
}

struct Upsampler {
    multipier: u8,
}

struct DSPChain<T> {
    data: Arc<[T]>,
}

pub fn upsample(data: Arc<[f32]>, original_frequency: u32) -> Arc<[f32]> {
    let upsampler = Upsampler::new(original_frequency);
    let low_pass = LowPassFilter::new(original_frequency);

    DSPChain::new(data)
        .flat_window(4, |window: Arc<[f32]>, start: usize, _end: usize| {
            upsampler.submit(window, start, _end)
        })
        .window(
            crate::dsp::LOW_PASS_FILTER_SIZE,
            |window: &Arc<[f32]>, start: usize, end: usize| low_pass.submit(window, start, end),
        )
        .collect()
}
