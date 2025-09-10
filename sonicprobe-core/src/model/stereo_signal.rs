use crate::model::{Signal, bit_depth::BitDepth, frequency::Frequency};

pub struct StereoSignal {
    pub left: Signal,
    pub right: Signal,
    pub sample_rate: Frequency,
    pub depth: BitDepth,
}

impl StereoSignal {
    #[must_use] pub fn samples_per_channel(&self) -> usize {
        self.left.len()
    }
}