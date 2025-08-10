use std::sync::Arc;

use serde::{Serialize, Serializer};

use crate::sonicprobe_error::SonicProbeError;

pub type Samples = f64;
pub type Signal = Arc<[Samples]>;
//pub type Frequency = u32;
pub type BitPrecision = u8;
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

impl Serialize for Frequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.to_hz())
    }
}

impl From<Frequency> for f64 {
    fn from(frequency: Frequency) -> Self {
        Self::from(frequency.to_hz())
    }
}


impl Frequency {
    pub fn new(value: u32) -> Result<Self, SonicProbeError> {
        match value {
            44100 => Ok(Self::CdQuality),
            48000 => Ok(Self::ProAudio),
            88200 => Ok(Self::HiResDouble),
            96000 => Ok(Self::DvdAudio),
            176_400 => Ok(Self::UltraHiRes),
            192_000 => Ok(Self::StudioMaster),
            _ => Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: format!("Currently {value}Hz sample rates is not supported"),
            }),
        }
    }

    pub const fn to_hz(self) -> u32 {
        match self {
            Self::CdQuality => 44100,
            Self::ProAudio => 48000,
            Self::HiResDouble => 88200,
            Self::DvdAudio => 96000,
            Self::UltraHiRes => 176_400,
            Self::StudioMaster => 192_000,
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::CdQuality => "CD Quality (44.1 kHz) - Standard for consumer audio",
            Self::ProAudio => "Professional Audio (48 kHz) - Industry standard for video/broadcast",
            Self::HiResDouble => "High-Res Double (88.2 kHz) - 2x CD rate for professional recording",
            Self::DvdAudio => "DVD Audio (96 kHz) - High-resolution consumer format",
            Self::UltraHiRes => "Ultra High-Res (176.4 kHz) - 4x CD rate for mastering",
            Self::StudioMaster => "Studio Master (192 kHz) - Highest professional standard",
        }
    }
}
