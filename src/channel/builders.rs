mod rms_builder;
mod dc_offset_builder;
mod peak_builder;
mod clipping_samples_builder;

#[derive(Clone, Copy)]
pub struct ClippingSamplesBuilder {
        count: i32
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