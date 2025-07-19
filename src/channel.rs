pub mod channel_builder;
mod rms_builder;
mod dc_offset_builder;
mod upsampler;
mod low_pass_filter;

use crate::audio_utils::to_dbfs;

pub struct Channel {
        rms: f32,
        peak: f32,
        clip_sample_count: i32,
        true_clip_sample_count: i32,
        dc_offset: f32,
        samples_count: i32,
        upsampled_samples_count: i32,
        true_peak: f32,
}

impl Channel {
        pub fn rms(&self) -> f32 {
                to_dbfs(self.rms)
        }

        pub fn peak(&self) -> f32 {
                to_dbfs(self.peak)
        }

        pub fn true_peak(&self) -> f32 {
                to_dbfs(self.true_peak)
        }

        pub fn clip_samples_quota(&self) -> f32 {
                (self.clip_sample_count as f32 / self.samples_count as f32) * 100.0
        }

        pub fn true_clip_samples_quota(&self) -> f32 {
                (self.true_clip_sample_count as f32 / self.upsampled_samples_count as f32) * 100.0
        }

        pub fn dc_offset(&self) -> f32 {
                self.dc_offset
        }

        pub fn crest_factor(&self) -> f32 {
                to_dbfs(self.peak / self.rms)
        }

        pub fn to_json_string(&self, father_tab: usize) -> String {
                let inner_tab: String = "\t".repeat(father_tab + 1);
                let output = [
                        format!("{}\"rms\": {},\n", inner_tab, self.rms()),
                        format!("{}\"peak\": {},\n", inner_tab, self.peak()),
                        format!("{}\"true_peak\": {},\n", inner_tab, self.true_peak()),
                        format!("{}\"clip_samples_quota\": {},\n", inner_tab, self.clip_samples_quota() / 100.0),
                        format!("{}\"true_clip_samples_quota\": {},\n", inner_tab, self.true_clip_samples_quota() / 100.0),
                        format!("{}\"dc_offset\": {},\n", inner_tab, self.dc_offset()),
                        format!("{}\"crest_factor\": {}", inner_tab, self.crest_factor())       
                ].concat();

                format!(
                        "{{\n{}\n{}}}",
                        output,
                        "\t".repeat(father_tab)
                )
        }
}