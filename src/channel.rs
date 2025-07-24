pub mod channel_builder;
mod upsampler;
mod low_pass_filter;
mod builders;

use crate::audio_utils::to_dbfs;

pub struct Channel {
        rms: f32,
        peak: f32,
        clipping_samples_count: u32,
        true_clipping_samples_count: u32,
        average_sample_value: f32,
        samples_count: u64,
        true_peak: f32,
        zero_crossing_rate: f32
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

        pub fn clipping_samples_quota(&self) -> f32 {
                (self.clipping_samples_count as f64 / self.samples_count as f64) as f32
        }

        pub fn true_clipping_samples_quota(&self) -> f32 {
                (self.true_clipping_samples_count as f64 / self.samples_count as f64) as f32
        }

        pub fn average_sample_value(&self) -> f32 {
                self.average_sample_value
        }

        pub fn crest_factor(&self) -> f32 {
                to_dbfs(self.peak / self.rms)
        }

        pub fn zero_crossing_rate(&self) -> f32 {
                self.zero_crossing_rate
        }

        pub fn to_json_string(&self, father_tab: usize) -> String {
                let inner_tab: String = "\t".repeat(father_tab + 1);
                let output = [
                        format!("{}\"rms\": {},\n", inner_tab, self.rms()),
                        format!("{}\"peak\": {},\n", inner_tab, self.peak()),
                        format!("{}\"true_peak\": {},\n", inner_tab, self.true_peak()),
                        format!("{}\"clipping_samples_quota\": {},\n", inner_tab, self.clipping_samples_quota()),
                        format!("{}\"true_clipping_samples_quota\": {},\n", inner_tab, self.true_clipping_samples_quota()),
                        format!("{}\"average_sample_value\": {},\n", inner_tab, self.average_sample_value()),
                        format!("{}\"crest_factor\": {}\n", inner_tab, self.crest_factor()),
                        format!("{}\"zero_crossing_rate\": {}", inner_tab, self.zero_crossing_rate()),       
                ].concat();

                format!(
                        "{{\n{}\n{}}}",
                        output,
                        "\t".repeat(father_tab)
                )
        }
}