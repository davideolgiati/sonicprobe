use crate::{
        builders::{
                ClippingSamplesBuilder, 
                PeakBuilder
        }, 
        circular_buffer::CircularBuffer
};

mod low_pass_filter;
mod old_upsampler;

mod upsampler;

const TARGET_FREQUENCY: u32 = 192000;
const NUMTAPS: usize = 128;

pub trait DSPStage {
        fn submit(&mut self, window: &[f32]);
        fn finalize(&self) -> Vec<f32>;
}

pub struct LowPassFilter {
        coeffs: [f32; 128],
        window: CircularBuffer<f32>,
        window_buffer: [f32; 128],
}

pub struct OldUpsampler {
        pub peak: f32,
        pub clipping_samples: u32,
        peak_builder: PeakBuilder,
        clipping_samples_builder: ClippingSamplesBuilder,
        window: CircularBuffer<f64>,
        factor: u8,
        lp_filter: LowPassFilter
}

pub struct Upsampler {
        multipier: u8,
        current_index: usize,
        signal: Vec<f32>
}