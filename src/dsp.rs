mod dsp_chain;
mod upsampler;
mod low_pass_filter;

const TARGET_FREQUENCY: u32 = 192000;
const NUMTAPS: usize = 128;

pub struct LowPassFilter {
        coeffs: [f32; NUMTAPS]
}

pub struct Upsampler {
        multipier: u8
}

pub struct DSPChain<T> {
    data: Vec<T>,
}
