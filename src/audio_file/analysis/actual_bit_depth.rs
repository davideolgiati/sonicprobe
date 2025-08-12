use crate::{
    audio_file::{types::BitDepth, Signal},
    constants::{MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, MAX_8_BIT},
    sonicprobe_error::SonicProbeError,
};

impl super::ActualBitDepth {
    #[inline]
    pub fn process(left: &Signal, right: &Signal, depth: BitDepth) -> Result<u8, SonicProbeError> {
        let factor = match depth {
            BitDepth::Legacy => MAX_8_BIT,
            BitDepth::CdStandard => MAX_16_BIT,
            BitDepth::Professional => MAX_24_BIT,
            BitDepth::StudioMaster => MAX_32_BIT,
        };

        let mut actual_depth = 0u8;

        for &sample in left.iter() {
            if sample == 0.0 {
                continue;
            }

            let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };

            let sample_depth: u8 = depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

            if sample_depth > actual_depth {
                actual_depth = sample_depth;
            }

            if actual_depth == depth.to_bits() {
                break;
            }
        }

        for &sample in right.iter() {
            if sample == 0.0 {
                continue;
            }

            let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };

            let sample_depth: u8 = depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

            if sample_depth > actual_depth {
                actual_depth = sample_depth;
            }

            if actual_depth == depth.to_bits() {
                break;
            }
        }

        Ok(actual_depth)
    }
}
