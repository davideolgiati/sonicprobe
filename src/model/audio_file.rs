use serde::Serialize;

use crate::model::{bit_depth::BitDepth, channel::Channel, decibel::Decibel, frequency::Frequency, Milliseconds};

#[derive(Serialize)]
pub struct AudioFile {
    pub left: Channel,
    pub right: Channel,
    pub samples_per_channel: usize,
    pub sample_rate: Frequency,
    pub duration: Milliseconds,
    pub stereo_correlation: f64,
    pub channels: u8,
    pub depth: BitDepth,
    pub true_depth: u8,
}

impl AudioFile {
    pub fn rms_balance(&self) -> Decibel {
        self.left.rms() - self.right.rms()
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(&self) {
            Ok(value) => value,
            Err(e) => format!("Error while serializing: {e:?}"),
        }
    }
}
