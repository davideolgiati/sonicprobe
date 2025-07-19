use crate::audio_utils::{cubic_interpolation, is_clipping, low_pass_filter};

pub struct Upsampler {
        pub signal: Vec<f32>,
        pub peak: f32,
        pub clipping_samples: i32,
        samples_seen: i32,
        window: Vec<f32>,
        factor: u8,
        lp_filter: Vec<f32>
}

impl Upsampler {
        pub fn new(factor: u8, original_frequency: u32) -> Upsampler {
                let cutoff_hz: u32 = original_frequency / 2;
                let lp_filter: Vec<f32> = low_pass_filter(
                        cutoff_hz as f32, 
                        (original_frequency * factor as u32) as f32, 
                        501
                );

                Upsampler {
                        signal: Vec::new(),
                        peak: f32::MIN,
                        clipping_samples: 0,
                        samples_seen: 0,
                        window: Vec::new(),
                        factor,
                        lp_filter
                }
        }

        fn add_new_sample(&mut self, sample: f32, upsampled: bool) {
                self.signal.push(sample);

                if !upsampled {
                        self.samples_seen += 1;
                }
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
                if self.window.len() == 4 {
                        self.window.remove(0);
                }

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
                
                self.add_new_sample(self.window[1], false);
                
                let upsamples = (1..self.factor)
                        .map(|k| k as f32 / self.factor as f32)
                        .map(|t| cubic_interpolation(
                                self.window[0], 
                                self.window[1], 
                                self.window[2], 
                                self.window[3], 
                                t)
                        ).collect::<Vec<f32>>();
                
                for upsample in upsamples {
                        self.add_new_sample(upsample, true)
                }
        }

        pub fn finalize(& mut self) {
                for _ in 1..3 {
                        self.add(self.window[3]);
                }

                let n = self.signal.len() as i32;
                let m = self.lp_filter.len() as i32;
                let half = m / 2;
                
                let mut output: Vec<f32> = Vec::new();
            
                for i in 0..n {
                        let mut acc: f32 = 0.0;
                        for j in 0..m {
                                let idx = i + j - half;
                                if idx >= 0 && idx < n {
                                        acc += self.signal[idx as usize] * self.lp_filter[j as usize];
                                }
                        }
                        output.push(acc);
                        self.update_stats(acc);
                }
                
                self.signal = output;
        }
}