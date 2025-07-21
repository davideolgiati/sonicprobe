use crate::{audio_utils::low_pass_filter, circular_buffer::CircularBuffer};

pub struct LowPassFilter {
        coeffs: Vec<f32>,
        window: CircularBuffer<f32>
}

impl LowPassFilter {
        pub fn new(original_frequency: u32, upsampling_factor: u32) -> LowPassFilter {
                let cutoff_hz: f32 = original_frequency as f32;
                let upsampled_freq: f32 = (original_frequency * upsampling_factor) as f32;
                let numtaps: i16 = 128;

                let coeffs: Vec<f32> = low_pass_filter(
                        cutoff_hz, 
                        upsampled_freq, 
                        numtaps
                );

                LowPassFilter { 
                        coeffs,
                        window: CircularBuffer::new(
                                numtaps as usize,
                                0.0
                        )
                }
        }

        pub fn filter(&mut self, sample: &f32) -> f32 {
                self.window.push(*sample);
                let window = self.window.collect();
                
                window.iter()
                    .zip(self.coeffs.iter())
                    .fold(0.0f32, |acc, (&w, &c)| acc + (w * c)) //TODO: da rivedere
        }
}