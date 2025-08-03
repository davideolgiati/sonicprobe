use std::sync::Arc;

use crate::{audio_utils::to_dbfs};

#[derive(Clone, Copy)]
pub struct Channel {
    rms: f32,
    peak: f32,
    clipping_samples_count: usize,
    pub true_clipping_samples_count: usize,
    dc_offset: f32,
    samples_count: u64,
    pub true_peak: f32,
    zero_crossing_rate: f32,
    dr: f32,
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

    pub fn dc_offset(&self) -> f32 {
        self.dc_offset
    }

    pub fn crest_factor(&self) -> f32 {
        to_dbfs(self.peak.abs() / self.rms)
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

    pub fn from_samples(samples: &Arc<[f32]>, sample_rate: u32, samples_per_channel: u64) -> Channel {
        let mut rms = 0.0f32;
        let mut dc_offset = 0.0f32;
        let mut dr_builder = super::analysis::DynamicRange::new(sample_rate);
        
        let duration = samples_per_channel as f32 / sample_rate as f32;
        
        rayon::scope(|s| {
            s.spawn(|_| coumpute_rms(samples, &mut rms));
            s.spawn(|_| coumpute_dc_offset(samples, samples_per_channel, &mut dc_offset));
        });
        
        dr_builder.add(samples);

        let zcr = super::analysis::ZeroCrossingRate::process(samples, duration);
        let clipping_samples_count = super::analysis::ClippingSamples::process(samples);
        let peak = super::analysis::Peak::process(samples);
        let dr = dr_builder.build(peak);

        Channel {
            rms,
            peak,
            true_peak: 0.0,
            samples_count: samples_per_channel,
            zero_crossing_rate: zcr,
            dc_offset,
            clipping_samples_count,
            true_clipping_samples_count: 0,
            dr,
        }
    }
}

fn coumpute_rms(samples: &Arc<[f32]>, output: &mut f32) {
    let mut builder = super::analysis::RootMeanSquare::new();
    samples.iter().for_each(|sample| builder.add(*sample));
    *output = builder.build()
}

fn coumpute_dc_offset(samples: &Arc<[f32]>, samples_count: u64, output: &mut f32) {
    let mut builder = super::analysis::DCOffset::new(samples_count);
    samples.iter().for_each(|sample| builder.add(*sample));
    *output = builder.build()
}
