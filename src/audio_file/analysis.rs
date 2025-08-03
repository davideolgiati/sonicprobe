mod clipping_samples;
mod dc_offset;
mod dynamic_range;
mod peak;
mod root_mean_square;
mod stereo_correlation;
mod actual_bit_depth;
mod zero_crossing_rate;

pub struct Peak;
pub struct ClippingSamples;
pub struct ZeroCrossingRate;
pub struct StereoCorrelation;
pub struct ActualBitDepth;

pub struct DCOffset {
    partials: Vec<f64>,
    count: u64,
}

pub struct RootMeanSquare {
    partials: Vec<f64>,
    count: u64,
}

pub struct DynamicRange {
    sample_frequency: u32,
    rms_avarage: f64,
}


