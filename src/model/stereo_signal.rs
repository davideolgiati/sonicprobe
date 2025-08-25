use crate::model::{Signal, bit_depth::BitDepth, frequency::Frequency};

pub struct StereoSignal {
    pub left: Signal,
    pub right: Signal,
    pub samples_per_channel: usize,
    pub sample_rate: Frequency,
    pub depth: BitDepth,
}
