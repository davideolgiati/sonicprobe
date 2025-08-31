use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Channel {
    pub(super) samples_count: u64,
    pub(super) dc_offset: f64,
    pub(super) true_peak: f64,
    pub(super) peak: f64,
    pub(super) rms: f64,
    pub(super) dr: f64,
    pub(super) true_clipping_samples_count: u64,
    pub(super) clipping_samples_count: u64,
    pub(super) zero_crossing_rate: u64,
}

impl Channel {
    #[inline]
    pub const fn dc_offset(&self) -> f64 {
        self.dc_offset
    }
    #[inline]
    pub const fn true_peak(&self) -> f64 {
        self.true_peak
    }
    #[inline]
    pub const fn peak(&self) -> f64 {
        self.peak
    }
    #[inline]
    pub const fn rms(&self) -> f64 {
        self.rms
    }
    #[inline]
    pub const fn dr(&self) -> f64 {
        self.dr
    }
    #[inline]
    pub const fn zero_crossing_rate(&self) -> u64 {
        self.zero_crossing_rate
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn clipping_samples_ratio(&self) -> f64 {
        self.clipping_samples_count as f64 / self.samples_count as f64
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn true_clipping_samples_ratio(&self) -> f64 {
        self.true_clipping_samples_count as f64 / self.samples_count as f64
    }

    pub const fn crest_factor(&self) -> f64 {
        self.peak - self.rms
    }
}
