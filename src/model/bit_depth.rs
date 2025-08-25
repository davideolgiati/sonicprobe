use serde::{Serialize, Serializer};

use crate::model::sonicprobe_error::SonicProbeError;

#[derive(Clone, Copy)]
pub enum BitDepth {
    Legacy,
    CdStandard,
    Professional,
    StudioMaster,
}

impl Serialize for BitDepth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.to_bits())
    }
}

impl BitDepth {
    pub fn new(value: u32) -> Result<Self, SonicProbeError> {
        match value {
            8 => Ok(Self::Legacy),
            16 => Ok(Self::CdStandard),
            24 => Ok(Self::Professional),
            32 => Ok(Self::StudioMaster),
            _ => Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: format!("Currently {value} bit depth is not supported"),
            }),
        }
    }

    pub const fn to_bits(self) -> u8 {
        match self {
            Self::Legacy => 8,
            Self::CdStandard => 16,
            Self::Professional => 24,
            Self::StudioMaster => 32,
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Legacy => " 8  bit - Legacy format",
            Self::CdStandard => "16  bit - CD standard",
            Self::Professional => "24  bit - Professional standard",
            Self::StudioMaster => "32  bit - Studio master quality",
        }
    }
}
