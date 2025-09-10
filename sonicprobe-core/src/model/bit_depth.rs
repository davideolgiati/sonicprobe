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

    #[must_use] pub const fn to_bits(self) -> u8 {
        match self {
            Self::Legacy => 8,
            Self::CdStandard => 16,
            Self::Professional => 24,
            Self::StudioMaster => 32,
        }
    }

    #[must_use] pub const fn description(self) -> &'static str {
        match self {
            Self::Legacy => " 8  bit - Legacy format",
            Self::CdStandard => "16  bit - CD standard",
            Self::Professional => "24  bit - Professional standard",
            Self::StudioMaster => "32  bit - Studio master quality",
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_legacy() {
        assert!(matches!(BitDepth::new(8).unwrap(), BitDepth::Legacy));
    }

    #[test]
    fn new_cd_standard() {
        assert!(matches!(BitDepth::new(16).unwrap(), BitDepth::CdStandard));
    }

    #[test]
    fn new_professional() {
        assert!(matches!(BitDepth::new(24).unwrap(), BitDepth::Professional));
    }

    #[test]
    fn new_studio_master() {
        assert!(matches!(BitDepth::new(32).unwrap(), BitDepth::StudioMaster));
    }

    #[test]
    fn new_unsupported() {
        assert!(BitDepth::new(12).is_err());
    }

    #[test]
    fn to_bits_legacy() {
        assert_eq!(BitDepth::Legacy.to_bits(), 8);
    }

    #[test]
    fn to_bits_cd_standard() {
        assert_eq!(BitDepth::CdStandard.to_bits(), 16);
    }

    #[test]
    fn to_bits_professional() {
        assert_eq!(BitDepth::Professional.to_bits(), 24);
    }

    #[test]
    fn to_bits_studio_master() {
        assert_eq!(BitDepth::StudioMaster.to_bits(), 32);
    }

    #[test]
    fn description_legacy() {
        assert_eq!(BitDepth::Legacy.description(), " 8  bit - Legacy format");
    }

    #[test]
    fn description_cd_standard() {
        assert_eq!(BitDepth::CdStandard.description(), "16  bit - CD standard");
    }

    #[test]
    fn description_professional() {
        assert_eq!(BitDepth::Professional.description(), "24  bit - Professional standard");
    }

    #[test]
    fn description_studio_master() {
        assert_eq!(BitDepth::StudioMaster.description(), "32  bit - Studio master quality");
    }

    #[test]
    fn serialize_legacy() {
        let json = serde_json::to_string(&BitDepth::Legacy).unwrap();
        assert_eq!(json, "8");
    }

    #[test]
    fn serialize_cd_standard() {
        let json = serde_json::to_string(&BitDepth::CdStandard).unwrap();
        assert_eq!(json, "16");
    }

    #[test]
    fn serialize_professional() {
        let json = serde_json::to_string(&BitDepth::Professional).unwrap();
        assert_eq!(json, "24");
    }

    #[test]
    fn serialize_studio_master() {
        let json = serde_json::to_string(&BitDepth::StudioMaster).unwrap();
        assert_eq!(json, "32");
    }
}