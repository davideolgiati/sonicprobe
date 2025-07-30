use crate::{
        audio_utils::catmull_rom_interpolation, 
        builders::{
                ClippingSamplesBuilder, 
                PeakBuilder
        }, 
        circular_buffer::CircularBuffer, 
        dsp::{
                LowPassFilter, 
                OldUpsampler
        }
};

impl OldUpsampler {
        pub fn new(original_frequency: u32) -> OldUpsampler {
                let factor: u8 = {
                        let ratio = (super::TARGET_FREQUENCY / original_frequency) as u8;
                        if ratio < 1 {
                                1
                        } else {
                                ratio
                        }
                };
                let lp_filter = LowPassFilter::new(
                        original_frequency, factor as u32
                );

                OldUpsampler {
                        peak: f32::MIN,
                        clipping_samples: 0,
                        peak_builder: PeakBuilder::new(),
                        clipping_samples_builder: ClippingSamplesBuilder::new(),
                        window: CircularBuffer::new(4, 0.0),
                        factor,
                        lp_filter
                }
        }

        fn add_new_sample(&mut self, sample: f32) {
                let filtered = self.lp_filter.filter(sample);
                self.clipping_samples_builder.add(filtered);
                self.peak_builder.add(filtered);
        }

        pub fn add(&mut self, sample: f32) {
                self.window.push(sample as f64);
                
                if self.window.len() < 4 {
                        return;
                }

                let window = self.window.collect().clone();
                
                self.add_new_sample(window[1] as f32);
                
                let factor = self.factor as f32;

                for k in 1..self.factor {
                        let interpolated = catmull_rom_interpolation(
                                window[0], 
                                window[1], 
                                window[2], 
                                window[3], 
                                k as f64 / factor as f64
                        );
                        self.add_new_sample(interpolated)
                }
        }

        pub fn finalize(& mut self) {
                let window = self.window.collect().clone();

                for _ in 1..3 {
                        self.add(window[3] as f32);
                }

                self.peak = self.peak_builder.build();
                self.clipping_samples = self.clipping_samples_builder.build();
        }
}