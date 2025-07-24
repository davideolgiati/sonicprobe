use crate::{audio_utils::low_pass_filter, circular_buffer::CircularBuffer};

const NUMTAPS: i16 = 64; // TODO: renedrelo parametrico

pub struct LowPassFilter {
        coeffs: [f32; 64],
        window: CircularBuffer<f32>,
        window_buffer: [f32; 64]
}

impl LowPassFilter {
        pub fn new(original_frequency: u32, upsampling_factor: u32) -> LowPassFilter {
                let cutoff_hz: f32 = (original_frequency as f32 * 0.8) / 2.0; // TODO: renederlo parametrico
                let upsampled_freq: f32 = (original_frequency * upsampling_factor) as f32;

                let mut coeffs: Vec<f32> = low_pass_filter(
                        cutoff_hz, 
                        upsampled_freq, 
                        NUMTAPS
                );

                coeffs.reverse();

                let mut coeffs_slice= [0.0f32; 64]; 
                coeffs_slice.copy_from_slice(&coeffs);

                LowPassFilter { 
                        coeffs: coeffs_slice,
                        window: CircularBuffer::new(
                                NUMTAPS as usize,
                                0.0
                        ),
                        window_buffer: [0.0f32; 64]
                }
        }

        pub fn filter(&mut self, sample: f32) -> f32 {
                self.window.push(sample);
                self.window_buffer.copy_from_slice(self.window.collect());

                dot_product(&self.coeffs, &self.window_buffer)
        }
}

#[inline]
fn dot_product(coeffs: &[f32; 64], samples: &[f32; 64]) -> f32 {
        debug_assert_eq!(coeffs.len(), samples.len());
    
        let mut sum = 0.0f32;
        
        let chunks = coeffs.len() / 4;
        let remainder = coeffs.len() % 4;
        
        for i in 0..chunks {
            let base_idx = i * 4;
            
            sum += coeffs[base_idx] * samples[base_idx]
                 + coeffs[base_idx + 1] * samples[base_idx + 1]
                 + coeffs[base_idx + 2] * samples[base_idx + 2]
                 + coeffs[base_idx + 3] * samples[base_idx + 3];
        }
        
        for i in (chunks * 4)..(chunks * 4 + remainder) {
            sum += coeffs[i] * samples[i];
        }
        
        sum
}