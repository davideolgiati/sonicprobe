use crate::{audio_utils::{cubic_interpolation, is_clipping}, channel::low_pass_filter::LowPassFilter, circular_buffer::CircularBuffer};

pub struct Upsampler {
        pub signal: Vec<f32>,
        pub peak: f32,
        pub clipping_samples: i32,
        samples_seen: i32,
        window: CircularBuffer<f32>,
        factor: u8,
        lp_filter: LowPassFilter
}

impl Upsampler {
        pub fn new(factor: u8, original_frequency: u32) -> Upsampler {
                let lp_filter = LowPassFilter::new(
                        original_frequency, factor as u32
                );

                Upsampler {
                        signal: Vec::new(),
                        peak: f32::MIN,
                        clipping_samples: 0,
                        samples_seen: 0,
                        window: CircularBuffer::new(4, 0.0),
                        factor,
                        lp_filter
                }
        }

        fn add_new_sample(&mut self, sample: f32, upsampled: bool) {
                let filtered_sample = self.lp_filter.filter(sample);

                self.signal.push(filtered_sample);

                if !upsampled {
                        self.samples_seen += 1;
                }

                self.update_stats(filtered_sample);
        }

        fn update_stats(&mut self, sample: f32) {
                if sample > self.peak {
                        self.peak = sample;
                }

                if is_clipping(sample) {
                        self.clipping_samples += 1
                }
        }

        fn update_upsampling_window(&mut self, sample: f32) {
                self.window.push(sample);

                if self.window.len() == 1 {
                        self.window.push(sample);
                }
        }

        pub fn add(&mut self, sample: f32) {
                self.update_upsampling_window(sample);
                
                if self.window.len() < 4 {
                        return;
                }
                
                self.add_new_sample(*self.window.at(1), false);
                
                let upsamples = (1..self.factor)
                        .map(|k| k as f32 / self.factor as f32)
                        .map(|t| cubic_interpolation(
                                *self.window.at(0), 
                                *self.window.at(1), 
                                *self.window.at(2), 
                                *self.window.at(3), 
                                t)
                        ).collect::<Vec<f32>>();
                
                for upsample in upsamples {
                        self.add_new_sample(upsample, true)
                }
        }

        pub fn finalize(& mut self) {
                for _ in 1..3 {
                        self.add(*self.window.at(3));
                }
        }
}