use crate::audio_utils::{cubic_interpolation, is_clipping};

pub struct Upsampler {
        pub signal: Vec<f64>,
        pub peak: f64,
        pub clipping_samples: i32,
        samples_seen: i32,
        window: Vec<f64>,
        factor: u8,
}

impl Upsampler {
        pub fn new(factor: u8) -> Upsampler {
                Upsampler {
                        signal: Vec::new(),
                        peak: f64::MIN,
                        clipping_samples: 0,
                        samples_seen: 0,
                        window: Vec::new(),
                        factor
                }
        }

        fn add_new_sample(&mut self, sample: f64, upsampled: bool) {
                if sample > self.peak {
                        self.peak = sample;
                }

                if is_clipping(sample) {
                        self.clipping_samples += 1
                }

                self.signal.push(sample);

                if !upsampled {
                        self.samples_seen += 1;
                }
        }

        fn update_upsampling_window(&mut self, sample: f64) {
                if self.window.len() == 4 {
                        self.window.remove(0);
                }

                self.window.push(sample);

                if self.window.len() == 1 {
                        self.window.push(sample);
                }
        }

        pub fn add(&mut self, sample: f64) {
                self.update_upsampling_window(sample);
                
                if self.window.len() < 4 {
                        return;
                }
                
                self.add_new_sample(self.window[1], false);
                
                let upsamples = (1..self.factor)
                        .map(|k| k as f64 / self.factor as f64)
                        .map(|t| cubic_interpolation(
                                self.window[0], 
                                self.window[1], 
                                self.window[2], 
                                self.window[3], 
                                t)
                        ).collect::<Vec<f64>>();
                
                for upsample in upsamples {
                        self.add_new_sample(upsample, true)
                }
        }

        pub fn finalize(& mut self) {
                for _ in 1..3 {
                        self.add(self.window[3])
                }
        }
}