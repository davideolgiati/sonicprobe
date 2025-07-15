use crate::{
        audio_utils::is_clipping, 
        channel::{
                dc_offset_builder::DCOffsetBuilder, rms_builder::RMSBuilder, upsampler::Upsampler, Channel
        }
};


pub struct ChannelBuilder {
        rms_builder: RMSBuilder,
        peak: f64,
        clip_samples_counter: i32,
        dc_offset_builder: DCOffsetBuilder,
        sample_counter: i32,
        upsampler: Upsampler
}

impl ChannelBuilder {
        pub fn new() -> ChannelBuilder {
                ChannelBuilder { 
                        rms_builder: RMSBuilder::new(), 
                        peak: f64::MIN, 
                        clip_samples_counter: 0, 
                        dc_offset_builder: DCOffsetBuilder::new(), 
                        sample_counter: 0,
                        upsampler: Upsampler::new(4)
                }
        }

        pub async fn add(&mut self, sample: f64) {
                self.rms_builder.add(sample);
                self.dc_offset_builder.add(sample);
                self.upsampler.add(sample);

                if is_clipping(sample) {
                        self.clip_samples_counter += 1
                }

                if self.peak < sample {
                        self.peak = sample;
                }

                self.sample_counter += 1;
        }

        pub async fn build(&mut self) -> Channel {
                self.upsampler.finalize();

                let rms = self.rms_builder.build();
                let dc_offset = self.dc_offset_builder.build();
                let true_peak = self.upsampler.peak;
                
                Channel {
                        rms : rms.await,
                        clip_sample_count: self.clip_samples_counter,
                        true_clip_sample_count: self.upsampler.clipping_samples,
                        peak: self.peak,
                        dc_offset: dc_offset.await,
                        samples_count: self.sample_counter,
                        upsampled_samples_count: self.upsampler.signal.len() as i32,
                        true_peak
                }
        }
    
}

