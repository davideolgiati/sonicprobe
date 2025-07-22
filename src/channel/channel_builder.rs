use crate::{
        audio_utils::is_clipping, 
        channel::{
                dc_offset_builder::DCOffsetBuilder, rms_builder::RMSBuilder, upsampler::Upsampler, Channel
        }
};


pub struct ChannelBuilder {
        rms_builder: RMSBuilder,
        peak: f32,
        clip_samples_counter: i32,
        dc_offset_builder: DCOffsetBuilder,
        sample_counter: i32,
        upsampler: Upsampler
}

impl ChannelBuilder {
        pub fn new(sample_rate: u32) -> ChannelBuilder {
                ChannelBuilder { 
                        rms_builder: RMSBuilder::new(), 
                        peak: f32::MIN, 
                        clip_samples_counter: 0, 
                        dc_offset_builder: DCOffsetBuilder::new(), 
                        sample_counter: 0,
                        upsampler: Upsampler::new(4, sample_rate)
                }
        }

        pub fn add(&mut self, sample: f32) {
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

        pub fn build(mut self) -> Channel {
                self.upsampler.finalize();

                let rms = self.rms_builder.build();
                let dc_offset = self.dc_offset_builder.build();
                let true_peak = self.upsampler.peak;
                
                Channel {
                        rms,
                        clip_sample_count: self.clip_samples_counter,
                        true_clip_sample_count: self.upsampler.clipping_samples,
                        peak: self.peak,
                        dc_offset,
                        samples_count: self.sample_counter,
                        upsampled_samples_count: self.upsampler.samples_count,
                        true_peak
                }
        }
    
}

