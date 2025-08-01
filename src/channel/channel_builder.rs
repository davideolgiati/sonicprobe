use std::sync::Arc;

use crate::builders::{
    ClippingSamplesBuilder, DCOffsetBuilder, DRBuilder, PeakBuilder, RMSBuilder,
    ZeroCrossingRateBuilder,
};
use crate::channel::Channel;

// TODO: questo va reso un metodo statico di channel
pub struct ChannelBuilder {}

impl ChannelBuilder {
    pub fn from_samples(samples: &Arc<[f32]>, sample_rate: u32, samples_count: u64) -> Channel {
        let mut rms = 0.0f32;
        let mut peak = 0.0f32;
        let mut clipping_samples_count = 0;
        let mut dc_offset = 0.0f32;
        let mut zcr = 0.0f32;
        let mut dr_builder = DRBuilder::new(sample_rate);

        let duration = samples_count as f32 / sample_rate as f32;

        rayon::scope(|s| {
            s.spawn(|_| coumpute_rms(samples, &mut rms));
            s.spawn(|_| peak = PeakBuilder::process(samples));
            s.spawn(|_| clipping_samples_count = ClippingSamplesBuilder::process(samples));
            s.spawn(|_| coumpute_dc_offset(samples, samples_count, &mut dc_offset));
            s.spawn(|_| zcr = ZeroCrossingRateBuilder::process(samples, duration));
            s.spawn(|_| dr_builder.add(samples));
        });

        let dr = dr_builder.build(peak);

        Channel {
            rms,
            peak,
            true_peak: 0.0,
            samples_count,
            zero_crossing_rate: zcr,
            dc_offset,
            clipping_samples_count,
            true_clipping_samples_count: 0,
            dr,
        }
    }
}

fn coumpute_rms(samples: &Arc<[f32]>, output: &mut f32) {
    let mut builder = RMSBuilder::new();
    samples.iter().for_each(|sample| builder.add(*sample));
    *output = builder.build()
}

fn coumpute_dc_offset(samples: &Arc<[f32]>, samples_count: u64, output: &mut f32) {
    let mut builder = DCOffsetBuilder::new(samples_count);
    samples.iter().for_each(|sample| builder.add(*sample));
    *output = builder.build()
}
