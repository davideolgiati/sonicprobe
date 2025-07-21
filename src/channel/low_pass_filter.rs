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

        pub fn filter(&mut self, sample: f32) -> f32 {
                self.window.push(sample);
                let filter_len = self.coeffs.len();
                let last_element_index = self.window.len();
                
                (0..filter_len).map(
                        |index| {
                                let buffer_idx = (last_element_index + filter_len - index) % filter_len;
                                self.window.at(buffer_idx) * self.coeffs[index]
                        }
                ).sum()
        }
}