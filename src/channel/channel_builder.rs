use crate::{
        audio_utils::is_clipping, 
        channel::{
                dc_offset_builder::DCOffsetBuilder, 
                rms_builder::RMSBuilder, 
                Channel
        }
};


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
                        sample_counter: 0,
                }
        }

        pub async fn add(&mut self, sample: f64) {
                self.rms_builder.add(sample);
                self.dc_offset_builder.add(sample);

                if is_clipping(sample) {
                        self.clip_samples_counter += 1
                }

                if self.peak < sample {
                        self.peak = sample;
                }

                self.sample_counter += 1;
        }

        pub async fn build(&mut self) -> Channel {
                let rms = self.rms_builder.build();
                let dc_offset = self.dc_offset_builder.build();
                
                Channel {
                        rms : rms.await,
                        clip_sample_count: self.clip_samples_counter,
                        peak: self.peak,
                        dc_offset: dc_offset.await,
                        samples_count: self.sample_counter
                }
        }
    
}

