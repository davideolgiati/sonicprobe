pub mod analysis;
pub mod channel;
pub mod types;

use std::fs::File;
use std::sync::Arc;

use flac::{ReadStream, Stream};
use serde::Serialize;

use crate::audio_file::analysis::ActualBitDepth;
use crate::audio_file::analysis::StereoCorrelation;
use crate::audio_file::channel::Channel;
use crate::audio_file::channel::new_channel_therad;
use crate::audio_file::types::BitPrecision;
use crate::audio_file::types::Frequency;
use crate::audio_file::types::Milliseconds;
use crate::audio_file::types::Signal;
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
    pub depth: BitPrecision,
    pub true_depth: BitPrecision,
}

impl AudioFile {
    pub fn new(stream: Stream<ReadStream<File>>) -> Result<Self, String> {
        let source = StereoSignal::from_flac(stream)?;
        let process_channel = new_channel_therad(source.sample_rate, source.samples_per_channel);

        let left_worker = process_channel(Arc::clone(&source.left));
        let right_worker = process_channel(Arc::clone(&source.right));

        let true_bit_depth = ActualBitDepth::process(&source.interleaved, source.depth);
        let stereo_correlation = StereoCorrelation::process(&source.left, &source.right);

        let left = match left_worker.join() {
            Ok(value) => value?,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };

        let right = match right_worker.join() {
            Ok(value) => value?,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };

        let signed_sample_count: i64 = match source.samples_per_channel.try_into() {
            Ok(value) => value,
            Err(e) => return Err(format!("AudioFile::new error: {e:?}")),
        };

        Ok(Self {
            left,
            right,
            channels: 2,
            stereo_correlation,
            true_depth: true_bit_depth,
            depth: source.depth,
            sample_rate: source.sample_rate,
            samples_per_channel: source.samples_per_channel,
            duration: signed_sample_count / i64::from(source.sample_rate),
        })
    }

    pub fn rms_balance(&self) -> f64 {
        self.left.rms - self.right.rms
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(&self) {
            Ok(value) => value,
            Err(e) => format!("Error while serializing: {e:?}"),
        }
    }
}
