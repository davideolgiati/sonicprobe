pub mod analysis;
pub mod channel;
pub mod types;

use std::fs::File;
use std::sync::Arc;
use std::thread;

use flac::{ReadStream, Stream};
use serde::Serialize;

use crate::audio_file::analysis::ActualBitDepth;
use crate::audio_file::analysis::StereoCorrelation;
use crate::audio_file::channel::Channel;
use crate::audio_file::channel::ChannelBuilder;
use crate::audio_file::types::BitDepth;
use crate::audio_file::types::Frequency;
use crate::audio_file::types::Milliseconds;
use crate::audio_file::types::Signal;
use crate::sonicprobe_error::SonicProbeError;
use crate::stereo_signal::StereoSignal;

#[derive(Serialize)]
pub struct AudioFile {
    pub left: Channel,
    pub right: Channel,
    pub samples_per_channel: u64,
    pub sample_rate: Frequency,
    pub duration: Milliseconds,
    pub stereo_correlation: f64,
    pub channels: u8,
    pub depth: BitDepth,
    pub true_depth: u8,
}

impl AudioFile {
    pub fn new(stream: Stream<ReadStream<File>>) -> Result<Self, SonicProbeError> {
        let source = StereoSignal::from_flac(stream)?;

        let left_handle = thread::spawn({
            let left_data = Arc::clone(&source.left);
            let sample_rate = source.sample_rate;
            move || ChannelBuilder::new(&left_data, sample_rate).build()
        });

        let right_handle = thread::spawn({
            let right_data = Arc::clone(&source.right);
            let sample_rate = source.sample_rate;
            move || ChannelBuilder::new(&right_data, sample_rate).build()
        });

        let true_bit_depth = ActualBitDepth::process(&source.interleaved, source.depth)?;
        let stereo_correlation = StereoCorrelation::process(&source.interleaved);

        let signed_sample_count: i64 = source.samples_per_channel.try_into()?;

        let left = left_handle.join()??;
        let right = right_handle.join()??;

        Ok(Self {
            left,
            right,
            channels: 2,
            stereo_correlation,
            true_depth: true_bit_depth,
            depth: source.depth,
            sample_rate: source.sample_rate,
            samples_per_channel: source.samples_per_channel,
            duration: signed_sample_count / i64::from(source.sample_rate.to_hz()),
        })
    }

    pub fn rms_balance(&self) -> f64 {
        self.left.rms() - self.right.rms()
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(&self) {
            Ok(value) => value,
            Err(e) => format!("Error while serializing: {e:?}"),
        }
    }
}
