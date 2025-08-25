use std::sync::Arc;

pub mod audio_file;
pub mod bit_depth;
pub mod builders;
pub mod channel;
pub mod frequency;
pub mod sonicprobe_error;
pub mod stereo_signal;

pub type Samples = f64;
pub type Signal = Arc<[Samples]>;
pub type Milliseconds = i64;

pub const TARGET_SAMPLE_RATE: f64 = 192_000.0;

pub const MAX_8_BIT: f64 = 127.0;
pub const MAX_16_BIT: f64 = 32767.0;
pub const MAX_24_BIT: f64 = 8_388_607.0;
pub const MAX_32_BIT: f64 = 2_147_483_647.0;

pub const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
