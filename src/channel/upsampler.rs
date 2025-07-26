use crate::{audio_utils::catmull_rom_interpolation, channel::{builders::{ClippingSamplesBuilder, PeakBuilder}, low_pass_filter::LowPassFilter}, circular_buffer::CircularBuffer};

const TARGET_FREQUENCY: u32 = 192000;

pub struct Upsampler {
        pub peak: f32,
        pub clipping_samples: u32,
        peak_builder: PeakBuilder,
        clipping_samples_builder: ClippingSamplesBuilder,
        window: CircularBuffer<f64>,
        factor: u8,
        lp_filter: LowPassFilter
}

impl Upsampler {
        pub fn new(original_frequency: u32) -> Upsampler {
                let factor: u8 = {
                        let ratio = (TARGET_FREQUENCY / original_frequency) as u8;
                        if ratio < 1 {
                                1
                        } else {
                                ratio
                        }
                };
                let lp_filter = LowPassFilter::new(
                        original_frequency, factor as u32
                );

                Upsampler {
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