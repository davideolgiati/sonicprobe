use crate::{audio_utils::{is_clipping, to_dbfs}, dc_offset_builder::DCOffsetBuilder, flac_file::Channel, rms_builder::RMSBuilder};

pub struct ChannelBuilder {
        rms_builder: RMSBuilder,
        peak: f64,
        clip_samples_counter: i32,
        dc_offset_builder: DCOffsetBuilder,
        sample_counter: i32
}

impl ChannelBuilder {
        pub fn new() -> ChannelBuilder {
                ChannelBuilder { 
                        rms_builder: RMSBuilder::new(), 
                        peak: f64::MIN, 
                        clip_samples_counter: 0, 
                        dc_offset_builder: DCOffsetBuilder::new(), 
                        sample_counter: 0
                }
        }

        pub async fn add(&mut self, sample: f64) {
                let rms_promise = self.rms_builder.add(sample);
                let dc_offset_promise = self.dc_offset_builder.add(sample);

                if is_clipping(sample) {
                        self.clip_samples_counter += 1
                }

                if self.peak < sample {
                        self.peak = sample;
                }

                self.sample_counter += 1;

                rms_promise.await;
                dc_offset_promise.await;
        }

        pub async fn build(&self) -> Channel {
                let rms = self.rms_builder.build();
                let dc_offset = self.dc_offset_builder.build();
                
                Channel {
                        rms : to_dbfs(rms.await),
                        clip_sample_count: self.clip_samples_counter,
                        peak: to_dbfs(self.peak),
                        dc_offset: dc_offset.await,
                        samples_count: self.sample_counter
                }
        }
    
}

