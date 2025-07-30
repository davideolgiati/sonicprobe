use crate::{audio_utils::low_pass_filter, dsp::{LowPassFilter, NUMTAPS}};

impl LowPassFilter {
    pub fn new(original_frequency: u32) -> Self {
        let cutoff_hz: f32 = (original_frequency as f32) / 2.0;
        let upsampled_freq: f32 = super::TARGET_FREQUENCY as f32;

        let mut coeffs: Vec<f32> = low_pass_filter(cutoff_hz, upsampled_freq, super::NUMTAPS);

        coeffs.reverse();

        let mut coeffs_slice = [0.0f32; super::NUMTAPS];
        coeffs_slice.copy_from_slice(&coeffs);

        Self {
            coeffs: coeffs_slice,
        }
    }

    pub fn submit(&self, window: &[f32]) -> Vec<f32> {
        let window_array: &[f32; super::NUMTAPS] = window.try_into().unwrap_or_else(|_| panic!("Window must be exactly {} elements, got {}", NUMTAPS, window.len()));
        vec![dot_product(&self.coeffs, window_array)]
    }
}

#[inline]
fn dot_product(coeffs: &[f32; super::NUMTAPS], samples: &[f32; super::NUMTAPS]) -> f32 {
    coeffs
        .chunks(4)
        .zip(samples.chunks(4))
        .fold(0.0, |acc: f32, (v1, v2)| {
            acc + v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2] + v1[3] * v2[3]
        })
}
