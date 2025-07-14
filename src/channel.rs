pub mod channel_builder;
mod rms_builder;
mod dc_offset_builder;

use crate::audio_utils::to_dbfs;

pub struct Channel {
        rms: f64,
        peak: f64,
        clip_sample_count: i32,
        dc_offset: f64,
        samples_count: i32,
}

impl Channel {
        pub fn rms(&self) -> f64 {
                to_dbfs(self.rms)
        }

        pub fn peak(&self) -> f64 {
                to_dbfs(self.peak)
        }

        pub fn clip_samples_quota(&self) -> f64 {
                (self.clip_sample_count as f64 / self.samples_count as f64) * 100.0
        }

        pub fn dc_offset(&self) -> f64 {
                self.dc_offset
        }

        pub fn crest_factor(&self) -> f64 {
                to_dbfs(self.peak / self.rms)
        }
}