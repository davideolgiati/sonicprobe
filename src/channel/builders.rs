mod rms_builder;
mod dc_offset_builder;
mod peak_builder;
mod clipping_samples_builder;
mod zero_crossing_rate_builder;

#[derive(Clone, Copy)]
pub struct ClippingSamplesBuilder {
        count: u32
}

#[derive(Clone)]
pub struct DCOffsetBuilder {
        partials: Vec<f64>,
        count: u64
}

#[derive(Clone, Copy)]
pub struct PeakBuilder {
        current_max: f32
}

#[derive(Clone)]
pub struct RMSBuilder {
        partials: Vec<f64>,
        count: u64
}

pub struct ZeroCrossingRateBuilder {
        count: u64,
        duration: f32,
        current_sign: i8
}