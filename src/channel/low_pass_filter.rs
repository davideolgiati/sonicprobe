use crate::{audio_utils::low_pass_filter, circular_buffer::CircularBuffer};

const NUMTAPS: i16 = 128;

pub struct LowPassFilter {
        coeffs: Vec<f32>,
        window: CircularBuffer<f32>
}

impl LowPassFilter {
        pub fn new(original_frequency: u32, upsampling_factor: u32) -> LowPassFilter {
                let cutoff_hz: f32 = original_frequency as f32 / 2.0;
                let upsampled_freq: f32 = (original_frequency * upsampling_factor) as f32;

                let mut coeffs: Vec<f32> = low_pass_filter(
                        cutoff_hz, 
                        upsampled_freq, 
                        NUMTAPS
                );

                coeffs.reverse();

                LowPassFilter { 
                        coeffs,
                        window: CircularBuffer::new(
                                NUMTAPS as usize,
                                0.0
                        )
                }
        }

        pub fn filter(&mut self, sample: f32) -> f32 {
                self.window.push(sample);
                let window = self.window.collect();
                
                dot_product(&self.coeffs, window)
        }
}

#[inline]
fn dot_product(coeffs: &[f32], samples: &[f32]) -> f32 {
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