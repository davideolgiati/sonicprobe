use serde::Serialize;
use std::sync::Arc;

use crate::{
    audio_file::{
        Frequency, Signal,
        analysis::{
            ClippingSamples, DCOffset, DynamicRange, Peak, RootMeanSquare, ZeroCrossingRate,
        },
    },
    dsp::upsample_chain,
    sonicprobe_error::SonicProbeError,
};

#[derive(Clone, Copy, Serialize)]
pub struct Channel {
    samples_count: u64,
    dc_offset: f64,
    true_peak: f64,
    peak: f64,
    rms: f64,
    dr: f64,
    true_clipping_samples_count: u64,
    clipping_samples_count: u64,
    zero_crossing_rate: f64,
}

impl Channel {
    #[inline]
    pub const fn dc_offset(&self) -> f64 {
        self.dc_offset
    }
    #[inline]
    pub const fn true_peak(&self) -> f64 {
        self.true_peak
    }
    #[inline]
    pub const fn peak(&self) -> f64 {
        self.peak
    }
    #[inline]
    pub const fn rms(&self) -> f64 {
        self.rms
    }
    #[inline]
    pub const fn dr(&self) -> f64 {
        self.dr
    }
    #[inline]
    pub const fn zero_crossing_rate(&self) -> f64 {
        self.zero_crossing_rate
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn clipping_samples_ratio(&self) -> f64 {
        self.clipping_samples_count as f64 / self.samples_count as f64
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn true_clipping_samples_ratio(&self) -> f64 {
        self.true_clipping_samples_count as f64 / self.samples_count as f64
    }

    pub fn crest_factor(&self) -> f64 {
        self.peak - self.rms
    }
}

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
        Self {
            signal: Arc::clone(signal),
            sample_rate,
            duration: signal.len() as f64 / f64::from(sample_rate),
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
    let rms = RootMeanSquare::process(samples)?;
    let zcr = ZeroCrossingRate::process(samples, builder.duration);
    let clipping_samples_count = ClippingSamples::process(samples);
    
    let upsampled_signal = upsample_chain(samples, builder.sample_rate)?;
    let true_peak = Peak::process(&upsampled_signal);
    let true_clipping_samples_count = ClippingSamples::process(&upsampled_signal);

    let dr = DynamicRange::process(samples, builder.sample_rate, true_peak)?;

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
