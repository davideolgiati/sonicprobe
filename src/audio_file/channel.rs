use serde::Serialize;
use std::{sync::Arc, thread};

use crate::{
    audio_file::{
        analysis::{ClippingSamples, DCOffset, DynamicRange, Peak, RootMeanSquare, ZeroCrossingRate}, Frequency, Signal
    },
    dsp::upsample,
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
    true_clipping_samples_count: usize,
    clipping_samples_count: usize,
    zero_crossing_rate: f64,
}

impl Channel {
    #[inline] pub const fn dc_offset(&self) -> f64 { self.dc_offset }
    #[inline] pub const fn true_peak(&self) -> f64 { self.true_peak }
    #[inline] pub const fn peak(&self) -> f64 { self.peak }
    #[inline] pub const fn rms(&self) -> f64 { self.rms }
    #[inline] pub const fn dr(&self) -> f64 { self.dr }
    #[inline] pub const fn zero_crossing_rate(&self) -> f64 { self.zero_crossing_rate }

    #[allow(clippy::cast_precision_loss)]
    pub fn clipping_samples_quota(&self) -> f64 {
        self.clipping_samples_count as f64 / self.samples_count as f64
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn true_clipping_samples_quota(&self) -> f64 {
        self.true_clipping_samples_count as f64 / self.samples_count as f64
    }

    pub fn crest_factor(&self) -> f64 {
        self.peak - self.rms
    }
}

#[repr(C)]
pub struct ChannelBuilder {
    signal: Signal,
    duration: f64,
    sample_rate: Frequency,
}

impl ChannelBuilder {
    pub fn new(signal: &Signal, sample_rate: Frequency) -> Self {
        Self {
            signal: Arc::clone(signal),
            sample_rate,
            duration: signal.len() as f64 / sample_rate as f64
        }
    }

    pub fn build_async(self) -> std::thread::JoinHandle<Result<Channel, SonicProbeError>> {
        thread::spawn(move || from_samples(&self))
    }
}

pub fn from_samples(builder: &ChannelBuilder) -> Result<Channel, SonicProbeError> {
    let samples = &builder.signal;
    let upsample_worker = upsample(Arc::clone(samples), builder.sample_rate);

    let dc_offset = DCOffset::process(samples)?;
    let rms = RootMeanSquare::process(samples)?;
    let zcr = ZeroCrossingRate::process(samples, builder.duration);
    let clipping_samples_count = ClippingSamples::process(samples);
    let peak = Peak::process(samples);

    let (true_peak, true_clipping_samples_count) = match upsample_worker.join() {
        Ok(values) => values?,
        Err(e) => {
            return Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: format!("{e:?}"),
            });
        }
    };

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
