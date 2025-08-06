use std::{sync::Arc, thread};
use serde::Serialize;

use crate::{
    audio_file::{analysis::{DCOffset, DynamicRange, RootMeanSquare}, Frequency, Signal},
    dsp::upsample,
};

#[derive(Clone, Copy, Serialize)]
pub struct Channel {
    pub samples_count: u64,
    pub dc_offset: f64,
    pub true_peak: f64,
    pub peak: f64,
    pub rms: f64,
    pub dr: f64,
    pub true_clipping_samples_count: usize,
    pub clipping_samples_count: usize,
    pub zero_crossing_rate: f32,
}

impl Channel {
    pub fn from_samples(
        samples: &Signal,
        sample_rate: Frequency,
        samples_per_channel: u64,
    ) -> Result<Self, String> {
        let upsample_worker = upsample(Arc::clone(samples), sample_rate);

        let duration = samples_per_channel as f32 / sample_rate as f32;

        let dc_offset = DCOffset::process(samples);
        let rms = RootMeanSquare::process(samples);
        let zcr = super::analysis::ZeroCrossingRate::process(samples, duration);
        let clipping_samples_count = super::analysis::ClippingSamples::process(samples);
        let peak = super::analysis::Peak::process(samples);
        
        let (true_peak, true_clipping_samples_count) = match upsample_worker.join() {
            Ok(values) => values,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };
        
        let dr = DynamicRange::process(samples, sample_rate, true_peak)?;

        Ok(Self {
            rms,
            peak,
            true_peak,
            samples_count: samples_per_channel,
            zero_crossing_rate: zcr,
            dc_offset,
            clipping_samples_count,
            true_clipping_samples_count,
            dr,
        })
    }

    pub fn clipping_samples_quota(&self) -> f64 {
        self.clipping_samples_count as f64 / self.samples_count as f64
    }

    pub fn true_clipping_samples_quota(&self) -> f64 {
        self.true_clipping_samples_count as f64 / self.samples_count as f64
    }

    pub fn crest_factor(&self) -> f64 {
        self.peak - self.rms
    }
}

pub fn new_channel_therad(
    sample_rate: Frequency,
    samples_per_channel: u64,
) -> impl Fn(Arc<[f64]>) -> std::thread::JoinHandle<Result<Channel, String>> {
    move |samples| {
        thread::spawn(move || Channel::from_samples(&samples, sample_rate, samples_per_channel))
    }
}