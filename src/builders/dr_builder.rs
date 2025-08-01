use std::sync::Arc;

use crate::{
    audio_utils::to_dbfs,
    builders::{DRBuilder, RMSBuilder},
};

impl DRBuilder {
    pub fn new(sample_frequency: u32) -> DRBuilder {
        DRBuilder {
            sample_frequency,
            rms_avarage: 0.0,
        }
    }

    #[inline]
    pub fn add(&mut self, samples: &Arc<[f32]>) {
        let chunk_size = (self.sample_frequency as f32 * 1.5).round() as usize;
        let reminder = samples.len() % chunk_size;
        let samples_end = samples.len() - reminder;

        let mut rms_array: Vec<f32> = samples[0..samples_end - chunk_size]
            .chunks(chunk_size)
            .zip(samples[chunk_size..samples_end].chunks(chunk_size))
            .map(|(last_chunk, chunk)| {
                let mut rms_builder = RMSBuilder::new();

                for sample in last_chunk {
                    rms_builder.add(*sample);
                }

                for sample in chunk {
                    rms_builder.add(*sample);
                }

                rms_builder.build()
            })
            .collect();

        let rms_end = (rms_array.len() as f32 * 0.2).round() as usize;
        rms_array.sort_by(|a, b| b.partial_cmp(a).unwrap());
        self.rms_avarage = rms_array[0..rms_end].iter().sum::<f32>() / rms_end as f32;
    }

    pub fn build(&self, peak: f32) -> f32 {
        to_dbfs(peak) - to_dbfs(self.rms_avarage)
    }
}
