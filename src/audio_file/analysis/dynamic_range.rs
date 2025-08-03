use std::process;

use crate::{audio_file::Signal, audio_utils::to_dbfs};

impl super::DynamicRange {
    pub fn new(sample_frequency: u32) -> super::DynamicRange {
        super::DynamicRange {
            sample_frequency,
            rms_avarage: 0.0,
        }
    }

    #[inline]
    pub fn add(&mut self, samples: &Signal) {
        let chunk_size = (self.sample_frequency as f32 * 1.5).round() as usize;
        let reminder = samples.len() % chunk_size;
        let samples_end = samples.len() - reminder;
        let analysable_samples = match samples.get(0..samples_end) {
            Some(samples) => samples,
            None => {
                println!("error: dynamic range can't slice samples in index 0 to {samples_end}");
                process::exit(1);
            }
        };

        let mut rms_array: Vec<f64> = 
            analysable_samples
            .chunks(chunk_size)
            .map(|chunk| {
                let mut rms_builder = super::RootMeanSquare::new();
                
                for sample in chunk {
                    rms_builder.add(*sample);
                }

                rms_builder.build()
            })
            .collect();

        let rms_end = (rms_array.len() as f32 * 0.2).round() as usize;
        rms_array.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let top_20_rms = match rms_array.get(0..rms_end) {
            Some(rms_slice) => rms_slice,
            None => {
                println!("error: dynamic range can't slice rms in index 0 to {rms_end}");
                process::exit(1);
            }
        };

        self.rms_avarage = top_20_rms.iter().sum::<f64>() / rms_end as f64;
    }

    pub fn build(&self, peak: f64) -> f64 {
        to_dbfs(peak) - to_dbfs(self.rms_avarage)
    }
}
