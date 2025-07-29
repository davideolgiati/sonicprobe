use crate::{audio_utils::low_pass_filter, circular_buffer::CircularBuffer};

const NUMTAPS: i16 = 128;

pub struct LowPassFilter {
    coeffs: [f32; 128],
    window: CircularBuffer<f32>,
    window_buffer: [f32; 128],
}

impl LowPassFilter {
    pub fn new(original_frequency: u32, upsampling_factor: u32) -> LowPassFilter {
        let cutoff_hz: f32 = (original_frequency as f32) / 2.0;
        let upsampled_freq: f32 = (original_frequency * upsampling_factor) as f32;

        let mut coeffs: Vec<f32> = low_pass_filter(cutoff_hz, upsampled_freq, NUMTAPS);

        coeffs.reverse();

        let mut coeffs_slice = [0.0f32; 128];
        coeffs_slice.copy_from_slice(&coeffs);

        LowPassFilter {
            coeffs: coeffs_slice,
            window: CircularBuffer::new(NUMTAPS as usize, 0.0),
            window_buffer: [0.0f32; 128],
        }
    }

    pub fn filter(&mut self, sample: f32) -> f32 {
        self.window.push(sample);
        self.window_buffer.copy_from_slice(self.window.collect());

        dot_product(&self.coeffs, &self.window_buffer)
    }
}

#[inline]
fn dot_product(coeffs: &[f32; 128], samples: &[f32; 128]) -> f32 {
    coeffs
        .chunks(32)
        .zip(samples.chunks(32))
        .fold(0.0, |acc: f32, (v1, v2)| {
            acc + v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2] + v1[3] * v2[3]
        })
}
