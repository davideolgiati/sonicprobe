use crate::{
    audio_file::{Signal, types::BitDepth},
    constants::{MAX_8_BIT, MAX_16_BIT, MAX_24_BIT, MAX_32_BIT},
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

            let sample_depth: u8 =
                depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

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

            let sample_depth: u8 =
                depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::{
        audio_file::{analysis::ActualBitDepth, types::BitDepth},
        constants::{MAX_16_BIT, MAX_8_BIT},
    };

    #[test]
    fn matching_size() {
        let left = [10, 20, 30, 111]
            .iter()
            .map(|val: &u8| f64::from(*val) / MAX_8_BIT)
            .collect();
        let right = [10, 20, 30, 111]
            .iter()
            .map(|val: &u8| f64::from(*val) / MAX_8_BIT)
            .collect();
        let depth = BitDepth::new(8).unwrap();
        let res = ActualBitDepth::process(&left, &right, depth).unwrap();

        assert_eq!(res, 8u8);
    }

        #[test]
    fn padded_data() {
        let left = [10, 20, 30, 111]
            .iter()
            .map(|val: &u16| f64::from(*val << 8) / MAX_16_BIT)
            .collect();
        let right = [10, 20, 30, 111]
            .iter()
            .map(|val: &u16| f64::from(*val << 8) / MAX_16_BIT)
            .collect();
        let depth = BitDepth::new(16).unwrap();
        let res = ActualBitDepth::process(&left, &right, depth).unwrap();

        assert_eq!(res, 8u8);
    }

            #[test]
    fn mixed_data() {
        let left = [10, 20, 30, 111]
            .iter()
            .map(|val: &u16| f64::from(*val << 1) / MAX_16_BIT)
            .collect();
        let right = [10, 20, 30, 111]
            .iter()
            .map(|val: &u16| f64::from(*val << 3) / MAX_16_BIT)
            .collect();
        let depth = BitDepth::new(16).unwrap();
        let res = ActualBitDepth::process(&left, &right, depth).unwrap();

        assert_eq!(res, 15u8);
    }
}
