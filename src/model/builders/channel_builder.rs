use std::sync::Arc;

use crate::{
    analysis::{ClippingSamples, DCOffset, DynamicRange, Peak, RootMeanSquare, ZeroCrossingRate},
    audio_utils::to_dbfs,
    dsp::upsample_chain,
    model::sonicprobe_error::SonicProbeError,
    model::{Signal, channel::Channel, frequency::Frequency},
};

#[repr(C)]
#[allow(clippy::module_name_repetitions)]
pub struct ChannelBuilder {
    signal: Signal,
    duration: f64,
    sample_rate: Frequency,
}

impl ChannelBuilder {
    #[allow(clippy::cast_precision_loss)]
    pub fn new(signal: &Signal, sample_rate: Frequency) -> Self {
        let duration = signal.len() as f64 / f64::from(sample_rate);

        Self {
            signal: Arc::clone(signal),
            sample_rate,
            duration,
        }
    }

    pub fn build(self) -> Result<Channel, SonicProbeError> {
        from_samples(&self)
    }
}

pub fn from_samples(builder: &ChannelBuilder) -> Result<Channel, SonicProbeError> {
    let samples = &builder.signal;

    let peak = Peak::process(samples);
    let dc_offset = DCOffset::process(samples)?;
    let rms = to_dbfs(RootMeanSquare::process(samples)?);
    let zcr = ZeroCrossingRate::process(samples, builder.sample_rate);
    let clipping_samples_count = ClippingSamples::process(samples);

    let (true_peak, true_clipping_samples_count) = upsample_chain(samples, builder.sample_rate)?;

    let dr = to_dbfs(DynamicRange::process(samples, builder.sample_rate));

    Ok(Channel {
        rms,
        peak,
        true_peak,
        samples_count: builder.signal.len() as u64,
        zero_crossing_rate: zcr,
        dc_offset,
        clipping_samples_count,
        true_clipping_samples_count,
        dr,
    })
}
