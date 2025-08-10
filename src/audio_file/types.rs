mod frequency;
mod bit_depth;

use std::sync::Arc;

pub type Samples = f64;
pub type Signal = Arc<[Samples]>;
pub type Milliseconds = i64;

#[derive(Clone, Copy)]
pub enum Frequency {
    CdQuality,
    ProAudio,
    HiResDouble,
    DvdAudio,
    UltraHiRes,
    StudioMaster,
}

#[derive(Clone, Copy)]
pub enum BitDepth {
    Legacy,
    CdStandard,
    Professional,
    StudioMaster,
}
