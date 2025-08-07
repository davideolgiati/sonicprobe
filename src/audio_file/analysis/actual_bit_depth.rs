use crate::{
    audio_file::Signal,
    constants::{MAX_8_BIT, MAX_16_BIT, MAX_24_BIT, MAX_32_BIT},
    sonicprobe_error::SonicProbeError,
};

impl super::ActualBitDepth {
    #[inline]
    pub fn process(interleaved: &Signal, depth: u8) -> Result<u8, SonicProbeError> {
        let factor = match depth {
            8 => MAX_8_BIT,
            16 => MAX_16_BIT,
            24 => MAX_24_BIT,
            32 => MAX_32_BIT,
            _ => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: "Unknown bit depth".to_owned(),
                });
            }
        };

        let mut res = 0u8;

        for sample in interleaved.iter() {
            if *sample == 0.0 {
                continue;
            }

            let reconstructed_value: i32 = unsafe { (*sample * factor).trunc().to_int_unchecked() };

            let actual_length = match u8::try_from(reconstructed_value.trailing_zeros()) {
                Ok(value) => depth - value,
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

            if res == depth {
                break;
            }
        }

        Ok(res)
    }
}
