mod rms_builder;
mod dc_offset_builder;
mod peak_builder;
mod clipping_samples_builder;
mod zero_crossing_rate_builder;

#[derive(Clone, Copy)]
pub struct ClippingSamplesBuilder {
        count: u32
}

#[derive(Clone, Copy)]
pub struct DCOffsetBuilder {
        sum: f64,
        count: usize
}

#[derive(Clone, Copy)]
pub struct PeakBuilder {
        current_max: f32
}

#[derive(Clone, Copy)]
pub struct RMSBuilder {
        sum: f64,
        count: usize
}

pub struct ZeroCrossingRateBuilder {
        count: u64,
        duration: f32,
        current_sign: i8
}