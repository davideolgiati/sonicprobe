use serde::Serialize;

use crate::model::{Milliseconds, bit_depth::BitDepth, channel::Channel, frequency::Frequency};

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
    pub const fn rms_balance(&self) -> f64 {
        self.left.rms() - self.right.rms()
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string_pretty(&self) {
            Ok(value) => value,
            Err(e) => format!("Error while serializing: {e:?}"),
        }
    }
}
