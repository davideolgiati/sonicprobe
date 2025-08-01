mod clipping_samples_builder;
mod dc_offset_builder;
mod dr_builder;
mod peak_builder;
mod rms_builder;
mod stereo_correlation_builder;
mod true_bit_depth_builder;
mod zero_crossing_rate_builder;

#[derive(Clone)]
pub struct DCOffsetBuilder {
    partials: Vec<f64>,
    count: u64,
}

pub struct PeakBuilder;
pub struct ClippingSamplesBuilder;
pub struct ZeroCrossingRateBuilder;
pub struct StereoCorrelationBuilder;

#[derive(Clone)]
pub struct RMSBuilder {
    partials: Vec<f64>,
    count: u64,
}

pub struct DRBuilder {
    sample_frequency: u32,
    rms_avarage: f32,
}

pub struct TrueBitDepthBuilder {
    min: u8,
    max: u8,
    avarage: f32,
    reported_depth: u8,
    sample_count: u64,
}

