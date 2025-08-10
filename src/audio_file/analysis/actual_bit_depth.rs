use crate::{
    audio_file::{types::BitDepth, Signal},
    constants::{MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, MAX_8_BIT},
    sonicprobe_error::SonicProbeError,
};

impl super::ActualBitDepth {
    #[inline]
    pub fn process(interleaved: &Signal, depth: BitDepth) -> Result<u8, SonicProbeError> {
        let factor = match depth {
            BitDepth::Legacy => MAX_8_BIT,
            BitDepth::CdStandard => MAX_16_BIT,
            BitDepth::Professional => MAX_24_BIT,
            BitDepth::StudioMaster => MAX_32_BIT,
        };

        let mut res = 0u8;

        for &sample in interleaved.iter() {
            if sample == 0.0 {
                continue;
            }

            let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };

            let actual_length = match u8::try_from(reconstructed_value.trailing_zeros()) {
                Ok(value) => depth.to_bits() - value,
                Err(_) => {
                    return Err(SonicProbeError {
                        location: format!("{}:{}", file!(), line!()),
                        message: format!(
                            "error safe casting form u32 to u8 value {}",
                            reconstructed_value.trailing_zeros()
                        ),
                    });
                }
            };

            if actual_length > res {
                res = actual_length;
            }

            if res == depth.to_bits() {
                break;
            }
        }

        Ok(res)
    }
}
