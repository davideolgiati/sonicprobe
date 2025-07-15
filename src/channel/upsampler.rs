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

        pub fn add(&mut self, sample: f64) {
                if sample > self.peak {
                        self.peak = sample;
                }

                if is_clipping(sample) {
                        self.clipping_samples += 1
                }

                self.samples_seen += 1;

                self.window.push(sample);
                if self.samples_seen == 1 {
                        self.window.push(sample);
                }

                if self.samples_seen > 3 {
                        self.signal.push(self.window[1]);
                        
                        for k in 1..self.factor{
                                let t = k as f64 / self.factor as f64;
                                let upsample = cubic_interpolation(
                                        self.window[0], 
                                        self.window[1], 
                                        self.window[2], 
                                        self.window[3], 
                                        t);
                                self.signal.push(upsample);

                                if upsample > self.peak {
                                        self.peak = upsample;
                                }

                                if is_clipping(upsample) {
                                        self.clipping_samples += 1
                                }
                        }
                        
                        self.window.remove(0);
                }

        }

        pub fn finalize(& mut self) {
                for _ in 1..3 {
                        self.add(self.window[3])
                }
        }
}