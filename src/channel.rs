pub mod channel_builder;

use crate::audio_utils::to_dbfs;

#[derive(Clone, Copy)]
pub struct Channel {
    rms: f32,
    peak: f32,
    clipping_samples_count: u32,
    pub true_clipping_samples_count: u32,
    dc_offset: f32,
    samples_count: u64,
    pub true_peak: f32,
    zero_crossing_rate: f32,
    dr: f32,
}

impl Channel {
    pub fn empty() -> Channel {
        Channel {
            rms: 0.0,
            peak: 0.0,
            clipping_samples_count: 0,
            true_clipping_samples_count: 0,
            dc_offset: 0.0,
            samples_count: 0,
            true_peak: 0.0,
            zero_crossing_rate: 0.0,
            dr: 0.0,
        }
    }

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

    pub fn dc_offset(&self) -> f32 {
        self.dc_offset
    }

    pub fn crest_factor(&self) -> f32 {
        to_dbfs(self.peak / self.rms)
    }

    pub fn zero_crossing_rate(&self) -> f32 {
        self.zero_crossing_rate
    }

    pub fn dr(&self) -> f32 {
        self.dr
    }

    pub fn as_json_string(&self, father_tab: usize) -> String {
        let inner_tab: String = "\t".repeat(father_tab + 1);
        let output = [
            format!("{}\"dynamic_range\": {},\n", inner_tab, self.dr()),
            format!("{}\"rms\": {},\n", inner_tab, self.rms()),
            format!("{}\"peak\": {},\n", inner_tab, self.peak()),
            format!("{}\"true_peak\": {},\n", inner_tab, self.true_peak()),
            format!(
                "{}\"clipping_samples_quota\": {},\n",
                inner_tab,
                self.clipping_samples_quota()
            ),
            format!(
                "{}\"true_clipping_samples_quota\": {},\n",
                inner_tab,
                self.true_clipping_samples_quota()
            ),
            format!("{}\"dc_offset\": {},\n", inner_tab, self.dc_offset()),
            format!("{}\"crest_factor\": {}\n", inner_tab, self.crest_factor()),
            format!(
                "{}\"zero_crossing_rate\": {}",
                inner_tab,
                self.zero_crossing_rate()
            ),
        ]
        .concat();

        format!("{{\n{}\n{}}}", output, "\t".repeat(father_tab))
    }
}
