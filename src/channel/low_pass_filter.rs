use crate::{audio_utils::low_pass_filter, circular_buffer::CircularBuffer};

pub struct LowPassFilter {
        coeffs: Vec<f32>,
        window: CircularBuffer<f32>
}

impl LowPassFilter {
        pub fn new(original_frequency: u32, upsampling_factor: u32) -> LowPassFilter {
                let cutoff_hz: f32 = (original_frequency / 2) as f32;
                let upsampled_freq: f32 = (original_frequency * upsampling_factor) as f32;
                let numtaps: i16 = 256;

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

                let mut acc = 0.0;
                let filter_len = self.coeffs.len();
                let last_element_index = self.window.len();
                
                for i in 0..filter_len {
                    let buffer_idx = (last_element_index + filter_len - i) % filter_len;
                    acc += self.window.at(buffer_idx) * self.coeffs[i];
                }
                
                acc
        }

        /*
        pub fn filter(&self, signal: &Vec<f32>) -> Vec<f32> {
                let n = signal.len() as i32;
                let m = self.coeffs.len() as i32;
                let half = m / 2;
                
                let mut output: Vec<f32> = Vec::new();
            
                for i in 0..n {
                        let mut acc: f32 = 0.0;
                        for j in 0..m {
                                let idx = i + j - half;
                                if idx >= 0 && idx < n {
                                        acc += signal[idx as usize] * self.coeffs[j as usize];
                                }
                        }
                        output.push(acc);
                        
                }
                
                output
        }
        */
}