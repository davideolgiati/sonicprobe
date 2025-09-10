use serde::{Serialize, Serializer};

use crate::model::sonicprobe_error::SonicProbeError;

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
        serializer.serialize_u64(self.to_hz() as u64)
    }
}

impl From<Frequency> for f64 {
    fn from(frequency: Frequency) -> Self {
        match frequency {
            Frequency::CdQuality => 44100.0,
            Frequency::ProAudio => 48000.0,
            Frequency::HiResDouble => 88200.0,
            Frequency::DvdAudio => 96000.0,
            Frequency::UltraHiRes => 176_400.0,
            Frequency::StudioMaster => 192_000.0,
        }
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

    #[must_use] pub const fn to_hz(self) -> usize {
        match self {
            Self::CdQuality => 44100,
            Self::ProAudio => 48000,
            Self::HiResDouble => 88200,
            Self::DvdAudio => 96000,
            Self::UltraHiRes => 176_400,
            Self::StudioMaster => 192_000,
        }
    }

    #[must_use] pub const fn description(self) -> &'static str {
        match self {
            Self::CdQuality => "44.1 kHz - Standard for consumer audio",
            Self::ProAudio => "48 kHz - Industry standard for video/broadcast",
            Self::HiResDouble => "88.2 kHz - 2x CD rate for professional recording",
            Self::DvdAudio => "96 kHz - High-resolution consumer format",
            Self::UltraHiRes => "176.4 kHz - 4x CD rate for mastering",
            Self::StudioMaster => "192 kHz - Highest professional standard",
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn new_cd_quality() {
        assert!(matches!(Frequency::new(44100).unwrap(), Frequency::CdQuality));
    }

    #[test]
    fn new_pro_audio() {
        assert!(matches!(Frequency::new(48000).unwrap(), Frequency::ProAudio));
    }

    #[test]
    fn new_hi_res_double() {
        assert!(matches!(Frequency::new(88200).unwrap(), Frequency::HiResDouble));
    }

    #[test]
    fn new_dvd_audio() {
        assert!(matches!(Frequency::new(96000).unwrap(), Frequency::DvdAudio));
    }

    #[test]
    fn new_ultra_hi_res() {
        assert!(matches!(Frequency::new(176_400).unwrap(), Frequency::UltraHiRes));
    }

    #[test]
    fn new_studio_master() {
        assert!(matches!(Frequency::new(192_000).unwrap(), Frequency::StudioMaster));
    }

    #[test]
    fn new_unsupported() {
        assert!(Frequency::new(22050).is_err());
    }

    #[test]
    fn to_hz_cd_quality() {
        assert_eq!(Frequency::CdQuality.to_hz(), 44100);
    }

    #[test]
    fn to_hz_pro_audio() {
        assert_eq!(Frequency::ProAudio.to_hz(), 48000);
    }

    #[test]
    fn to_hz_hi_res_double() {
        assert_eq!(Frequency::HiResDouble.to_hz(), 88200);
    }

    #[test]
    fn to_hz_dvd_audio() {
        assert_eq!(Frequency::DvdAudio.to_hz(), 96000);
    }

    #[test]
    fn to_hz_ultra_hi_res() {
        assert_eq!(Frequency::UltraHiRes.to_hz(), 176_400);
    }

    #[test]
    fn to_hz_studio_master() {
        assert_eq!(Frequency::StudioMaster.to_hz(), 192_000);
    }

    #[test]
    fn description_cd_quality() {
        assert_eq!(Frequency::CdQuality.description(), "44.1 kHz - Standard for consumer audio");
    }

    #[test]
    fn description_pro_audio() {
        assert_eq!(Frequency::ProAudio.description(), "48 kHz - Industry standard for video/broadcast");
    }

    #[test]
    fn description_hi_res_double() {
        assert_eq!(Frequency::HiResDouble.description(), "88.2 kHz - 2x CD rate for professional recording");
    }

    #[test]
    fn description_dvd_audio() {
        assert_eq!(Frequency::DvdAudio.description(), "96 kHz - High-resolution consumer format");
    }

    #[test]
    fn description_ultra_hi_res() {
        assert_eq!(Frequency::UltraHiRes.description(), "176.4 kHz - 4x CD rate for mastering");
    }

    #[test]
    fn description_studio_master() {
        assert_eq!(Frequency::StudioMaster.description(), "192 kHz - Highest professional standard");
    }

    #[test]
    fn serialize_cd_quality() {
        let json = serde_json::to_string(&Frequency::CdQuality).unwrap();
        assert_eq!(json, "44100");
    }

    #[test]
    fn serialize_pro_audio() {
        let json = serde_json::to_string(&Frequency::ProAudio).unwrap();
        assert_eq!(json, "48000");
    }

    #[test]
    fn serialize_hi_res_double() {
        let json = serde_json::to_string(&Frequency::HiResDouble).unwrap();
        assert_eq!(json, "88200");
    }

    #[test]
    fn serialize_dvd_audio() {
        let json = serde_json::to_string(&Frequency::DvdAudio).unwrap();
        assert_eq!(json, "96000");
    }

    #[test]
    fn serialize_ultra_hi_res() {
        let json = serde_json::to_string(&Frequency::UltraHiRes).unwrap();
        assert_eq!(json, "176400");
    }

    #[test]
    fn serialize_studio_master() {
        let json = serde_json::to_string(&Frequency::StudioMaster).unwrap();
        assert_eq!(json, "192000");
    }

    #[test]
    fn from_cd_quality() {
        let f: f64 = Frequency::CdQuality.into();
        assert_eq!(f, 44100.0);
    }

    #[test]
    fn from_pro_audio() {
        let f: f64 = Frequency::ProAudio.into();
        assert_eq!(f, 48000.0);
    }

    #[test]
    fn from_hi_res_double() {
        let f: f64 = Frequency::HiResDouble.into();
        assert_eq!(f, 88200.0);
    }

    #[test]
    fn from_dvd_audio() {
        let f: f64 = Frequency::DvdAudio.into();
        assert_eq!(f, 96000.0);
    }

    #[test]
    fn from_ultra_hi_res() {
        let f: f64 = Frequency::UltraHiRes.into();
        assert_eq!(f, 176_400.0);
    }

    #[test]
    fn from_studio_master() {
        let f: f64 = Frequency::StudioMaster.into();
        assert_eq!(f, 192_000.0);
    }
}