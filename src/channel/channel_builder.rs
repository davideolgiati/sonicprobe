use crate::channel::builders::{
        ClippingSamplesBuilder, AverageSampleValueBuilder, PeakBuilder, RMSBuilder, ZeroCrossingRateBuilder
};
use crate::channel::upsampler::Upsampler;
use crate::channel::Channel;


pub struct ChannelBuilder {
        rms_builder: RMSBuilder,
        peak_builder: PeakBuilder,
        clipping_samples_builder: ClippingSamplesBuilder,
        average_sample_value_builder: AverageSampleValueBuilder,
        sample_counter: u64,
        upsampler: Upsampler,
        zero_crossing_rate_builder: ZeroCrossingRateBuilder
}

impl ChannelBuilder {
        pub fn new(sample_rate: u32, samples_per_channel: u64) -> ChannelBuilder {
                ChannelBuilder { 
                        rms_builder: RMSBuilder::new(), 
                        peak_builder: PeakBuilder::new(), 
                        clipping_samples_builder: ClippingSamplesBuilder::new(), 
                        average_sample_value_builder: AverageSampleValueBuilder::new(), 
                        sample_counter: samples_per_channel,
                        upsampler: Upsampler::new(4, sample_rate, samples_per_channel),
                        zero_crossing_rate_builder: ZeroCrossingRateBuilder::new(samples_per_channel)
                }
        }

        #[inline]
        pub fn add(&mut self, sample: f32) {
                self.rms_builder.add(sample);
                self.average_sample_value_builder.add(sample);
                self.upsampler.add(sample);
                self.peak_builder.add(sample);
                self.clipping_samples_builder.add(sample);
                self.zero_crossing_rate_builder.add(sample);
        }

        pub fn build(mut self) -> Channel {
                self.upsampler.finalize();

                let rms = self.rms_builder.build();
                let peak = self.peak_builder.build();
                let clipping_samples_count = self.clipping_samples_builder.build();
                let average_sample_value = self.average_sample_value_builder.build();
                let true_peak = self.upsampler.peak;
                let zero_crossing_rate = self.zero_crossing_rate_builder.build();
                
                Channel {
                        rms,
                        clipping_samples_count,
                        true_clipping_samples_count: self.upsampler.clipping_samples,
                        peak,
                        average_sample_value,
                        samples_count: self.sample_counter,
                        upsampled_samples_count: self.upsampler.samples_count,
                        true_peak,
                        zero_crossing_rate
                }
        }
    
}

