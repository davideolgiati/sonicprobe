pub const UPSAMPLE_TARGET_FREQUENCY: u32 = 192_000;
pub const LOW_PASS_FILTER_SIZE: usize = 48;

pub const MAX_8_BIT: f32 = i8::MAX as f32;
pub const MAX_16_BIT: f32 = i16::MAX as f32;
pub const MAX_24_BIT: f32 = ((1 << 23) - 1) as f32;
pub const MAX_32_BIT: f32 = i32::MAX as f32;

pub const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];