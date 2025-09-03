use serde::Serialize;

use crate::model::{decibel::Decibel, dynamic_range::DynamicRange};

#[derive(Clone, Copy, Serialize)]
pub struct Channel {
    pub(super) samples_count: u64,
    pub(super) dc_offset: f64,
    pub(super) true_peak: Decibel,
    pub(super) peak: Decibel,
    pub(super) rms: Decibel,
    pub(super) dr: DynamicRange,
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
    pub const fn true_peak(&self) -> Decibel {
        self.true_peak
    }
    #[inline]
    pub const fn peak(&self) -> Decibel {
        self.peak
    }
    #[inline]
    pub const fn rms(&self) -> Decibel {
        self.rms
    }
    #[inline]
    pub const fn dr(&self) -> DynamicRange {
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

    pub fn crest_factor(&self) -> Decibel {
        self.peak - self.rms
    }
}
