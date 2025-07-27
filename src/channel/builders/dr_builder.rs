use crate::{audio_utils::to_dbfs, channel::builders::{DRBuilder, RMSBuilder}};

impl DRBuilder {
        pub fn new(sample_frequency: u32) -> DRBuilder {
                DRBuilder { 
                        sample_frequency, 
                        rms_avarage: 0.0
                }
        }

        pub fn add(&mut self, samples: &[f32]) {
                let mut rms_array: Vec<f32> = Vec::new();
                let chunk_size = (self.sample_frequency as f32 * 1.5).round() as usize;
                let reminder = samples.len() % chunk_size;
                let samples_end = samples.len() - reminder;
                let mut last_chunk: Vec<f32> = Vec::new();

                for chunk in samples[0..samples_end].chunks(chunk_size) {
                        if !last_chunk.is_empty() {
                                let mut current_chunk = last_chunk.clone();
                                current_chunk.append(&mut chunk.to_vec());
                                let mut rms_builder = RMSBuilder::new(chunk_size as u64);
                                for sample in current_chunk {
                                        rms_builder.add(sample);
                                }
                                rms_array.push(rms_builder.build())
                        }
                        last_chunk = chunk.to_vec()
                }

                let rms_end = (rms_array.len() as f32 * 0.2).round() as usize;
                rms_array.sort_by(|a, b| b.partial_cmp(a).unwrap());
                self.rms_avarage = rms_array[0..rms_end].iter().sum::<f32>() / rms_end as f32;
        }

        pub fn build(&self, peak: f32) -> f32 {
                to_dbfs(peak) - to_dbfs(self.rms_avarage)
        }

}