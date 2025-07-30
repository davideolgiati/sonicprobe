use crate::{audio_utils::catmull_rom_interpolation, dsp::{DSPStage, Upsampler}};

impl Upsampler {
        pub fn new(original_frequency: u32, original_size: u64) -> Upsampler {
                let multipier: u8 = {
                        let ratio = (super::TARGET_FREQUENCY / original_frequency) as u8;
                        if ratio < 1 {
                                1
                        } else {
                                ratio
                        }
                };

                let new_size = original_size * multipier as u64;

                Upsampler {
                        multipier,
                        current_index: 0,
                        signal: Vec::with_capacity(new_size as usize)
                }
        }
}

impl DSPStage for Upsampler {
        fn submit(&mut self, window: &[f32]){
                self.signal[self.current_index] = window[1];
                self.current_index += 1;          

                for k in 1..self.multipier {
                        let interpolated = catmull_rom_interpolation(
                                window[0] as f64, 
                                window[1] as f64, 
                                window[2] as f64, 
                                window[3] as f64, 
                                k as f64 / self.multipier as f64
                        );
                        self.signal[self.current_index] = interpolated;
                        self.current_index += 1  
                }
        }

        fn finalize(&self) -> Vec<f32>{
                self.signal.clone()
        }
}