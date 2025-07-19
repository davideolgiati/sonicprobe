use crate::audio_utils::low_pass_filter;

pub struct LowPassFilter {
        coeffs: Vec<f32>,
        num_taps: i16
}

impl LowPassFilter {
        pub fn new(original_frequency: u32, upsampling_factor: u32) -> LowPassFilter {
                let cutoff_hz: f32 = (original_frequency / 2) as f32;
                let upsampled_freq: f32 = (original_frequency * upsampling_factor) as f32;

                let coeffs: Vec<f32> = low_pass_filter(
                        cutoff_hz, 
                        upsampled_freq, 
                        501
                );

                LowPassFilter { 
                        coeffs,
                        num_taps: 501
                }
        }

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
}