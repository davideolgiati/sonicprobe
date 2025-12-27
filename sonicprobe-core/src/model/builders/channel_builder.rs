use std::sync::Arc;

use crate::{
    analysis::peak::update_peak_value, dsp::{
        analysis::{
            clipping::update_clipping_count, dc_offset::calculate_dc_offset,
            dynamic_range::calculate_dynamic_range,
            root_mean_square::compute_root_mean_square,
            zero_crossing_rate::calculate_zero_crossing_rate,
        },
        upsample_chain,
    }, model::{Signal, channel::Channel, decibel::Decibel, dynamic_range::DynamicRange, frequency::Frequency, sonicprobe_error::SonicProbeError}
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
    #[must_use] pub fn new(signal: &Signal, sample_rate: Frequency) -> Self {
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

    let peak = find_signal_peak(samples);
    let dc_offset = calculate_dc_offset(samples)?;
    let rms = Decibel::new(compute_root_mean_square(samples)?);
    let zcr = calculate_zero_crossing_rate(samples, builder.sample_rate);
    let clipping_samples_count = count_clipping_samples(samples);

    let (true_peak, true_clipping_samples_count) = upsample_chain(samples, builder.sample_rate)?;

    let dr = DynamicRange::from(calculate_dynamic_range(samples, builder.sample_rate));

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

fn count_clipping_samples(samples: &Signal) -> u64 {
    let mut clipping_samples_count = 0u64;

    for sample in samples.iter() {
        match update_clipping_count(&clipping_samples_count, sample) {
            Some(result) => clipping_samples_count = result,
            None => {}
        }
    }

    clipping_samples_count
}

fn find_signal_peak(samples: &Signal) -> Decibel {
    let mut peak = f64::MIN;

    for &sample in samples.iter() {
        match update_peak_value(&peak, &sample.abs()) {
            Some(result) => peak = result,
            None => {}
        }
    }

    Decibel::new(peak)
}