use crate::channel::builders::{
        ClippingSamplesBuilder, DCOffsetBuilder, PeakBuilder, RMSBuilder, ZeroCrossingRateBuilder
};
use crate::channel::upsampler::Upsampler;
use crate::channel::Channel;

struct UpsamplerOutput {
        true_peak: f32,
        true_clipping_samples: u32,
}

pub struct ChannelBuilder {}

impl ChannelBuilder {
        #[inline]
        pub fn from_samples(samples: &[f32], sample_rate: u32, samples_count: u64) -> Channel {
                let mut rms = 0.0f32;
                let mut peak = 0.0f32;
                let mut clipping_samples_count = 0;
                let mut dc_offset = 0.0f32;
                let mut zero_crossing_rate = 0.0f32;
                let mut upsampler_output = UpsamplerOutput{ true_peak: 0.0, true_clipping_samples: 0 };

                rayon::scope(|s| {
                        s.spawn(|_| coumpute_rms(samples, samples_count, &mut rms));
                        s.spawn(|_| coumpute_peak(samples, &mut peak));
                        s.spawn(|_| count_clipping_samples(samples, &mut clipping_samples_count));
                        s.spawn(|_| coumpute_dc_offset(samples, samples_count, &mut dc_offset));
                        s.spawn(|_| coumpute_zero_crossing_rate(samples, samples_count, sample_rate,&mut zero_crossing_rate));
                        s.spawn(|_| compute_upsampled_statistics(samples, sample_rate, &mut upsampler_output));
                });

                let true_clipping_samples_count = upsampler_output.true_clipping_samples;
                let true_peak = upsampler_output.true_peak;

                Channel {
                        rms,
                        peak,
                        true_peak,
                        samples_count,
                        zero_crossing_rate,
                        dc_offset,
                        clipping_samples_count,
                        true_clipping_samples_count
                }
        }
    
}

fn coumpute_rms(samples: &[f32], samples_count: u64, output: &mut f32) {
        let mut builder = RMSBuilder::new(samples_count);
        for sample in samples {
                builder.add(*sample);
        }
        *output = builder.build()
}

fn coumpute_peak(samples: &[f32], output: &mut f32) {
        let mut builder = PeakBuilder::new();
        for sample in samples {
                builder.add(*sample);
        }
        *output = builder.build()
}

fn count_clipping_samples(samples: &[f32], output: &mut u32) {
        let mut rms_builder = ClippingSamplesBuilder::new();
        for sample in samples {
                rms_builder.add(*sample);
        }
        *output = rms_builder.build()
}

fn coumpute_dc_offset(samples: &[f32], samples_count: u64, output: &mut f32) {
        let mut builder = DCOffsetBuilder::new(samples_count);
        for sample in samples {
                builder.add(*sample);
        }
        *output = builder.build()
}

fn coumpute_zero_crossing_rate(samples: &[f32], samples_count: u64, sample_rate: u32, output: &mut f32) {
        let mut builder = ZeroCrossingRateBuilder::new(samples_count as f32 / sample_rate as f32);
        for sample in samples {
                builder.add(*sample);
        }
        *output = builder.build()
}

fn compute_upsampled_statistics(samples: &[f32], original_frequency: u32, output: &mut UpsamplerOutput) {
        let mut builder = Upsampler::new(original_frequency);
        for sample in samples {
                builder.add(*sample);
        }
        builder.finalize();
        output.true_clipping_samples = builder.clipping_samples;
        output.true_peak = builder.peak;
}
