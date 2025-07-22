use crate::{audio_utils::{catmull_rom_interpolation, is_clipping}, channel::low_pass_filter::LowPassFilter, circular_buffer::CircularBuffer};

pub struct Upsampler {
        pub peak: f32,
        pub clipping_samples: i32,
        pub samples_count: i32,
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
                        peak: f32::MIN,
                        clipping_samples: 0,
                        samples_count: 0,
                        window: CircularBuffer::new(4, 0.0),
                        factor,
                        lp_filter
                }
        }

        fn add_new_sample(&mut self, sample: f32) {
                let filtered_sample = self.lp_filter.filter(sample);
                self.samples_count += 1;

                if filtered_sample > self.peak {
                        self.peak = filtered_sample;
                }

                if is_clipping(filtered_sample) {
                        self.clipping_samples += 1
                }
        }

        fn update_upsampling_window(&mut self, sample: f32) {
                self.window.push(sample);

                if self.window.len() != 1 {
                        return;
                }
                
                self.window.push(sample);
        }

        pub fn add(&mut self, sample: f32) {
                self.update_upsampling_window(sample);
                
                if self.window.len() == 4 {
                        let window = self.window.collect().clone();
                        
                        self.add_new_sample(window[1]);
                        
                        let factor = self.factor as f32;

                        (1..self.factor)
                                .map(|k| catmull_rom_interpolation(
                                        window[0], 
                                        window[1], 
                                        window[2], 
                                        window[3], 
                                        k as f32 / factor
                                ))
                                .for_each(|sample| {
                                        self.add_new_sample(sample)
                                });
                }
        }

        pub fn finalize(& mut self) {
                let window = self.window.collect().clone();

                for _ in 1..3 {
                        self.add(window[3]);
                }
        }
}