use std::sync::Arc;

mod dsp_chain;
mod low_pass_filter;
mod upsampler;

const TARGET_FREQUENCY: u32 = 192000;
pub const LOW_PASS_FILTER_SIZE: usize = 512;

pub struct LowPassFilter {
    coeffs: [f32; LOW_PASS_FILTER_SIZE],
}

pub struct Upsampler {
    multipier: u8,
}

pub struct DSPChain<T> {
    data: Arc<[T]>,
}
