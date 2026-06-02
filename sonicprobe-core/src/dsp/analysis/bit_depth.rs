use crate::model::{
    MAX_8_BIT, MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, bit_depth::BitDepth,
    sonicprobe_error::SonicProbeError, stereo_signal::StereoSignal,
};

#[inline]
pub fn calculate_true_depth(source: &StereoSignal) -> Result<u8, SonicProbeError> {
    let factor = match source.depth {
        BitDepth::Legacy => MAX_8_BIT,
        BitDepth::CdStandard => MAX_16_BIT,
        BitDepth::Professional => MAX_24_BIT,
        BitDepth::StudioMaster => MAX_32_BIT,
    };

    let mut actual_depth = 0u8;

    for &sample in source.left.iter() {
        if sample == 0.0 {
            continue;
        }

        // `to_int_unchecked::<i32>()` already truncates toward zero — an explicit
        // `.trunc()` lowers to a per-sample libm call for nothing. Drop it.
        let reconstructed_value: i32 = unsafe { (sample * factor).to_int_unchecked() };

        // A non-zero sample below 0.5 LSB reconstructs to 0; `trailing_zeros(0)`
        // is 32 and would underflow `to_bits() - …`. Such a sample carries no
        // depth information, so skip it.
        if reconstructed_value == 0 {
            continue;
        }

        let sample_depth: u8 =
            source.depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

        if sample_depth > actual_depth {
            actual_depth = sample_depth;
        }

        if actual_depth == source.depth.to_bits() {
            break;
        }
    }

    for &sample in source.right.iter() {
        if sample == 0.0 {
            continue;
        }

        // `to_int_unchecked::<i32>()` already truncates toward zero — an explicit
        // `.trunc()` lowers to a per-sample libm call for nothing. Drop it.
        let reconstructed_value: i32 = unsafe { (sample * factor).to_int_unchecked() };

        // A non-zero sample below 0.5 LSB reconstructs to 0; `trailing_zeros(0)`
        // is 32 and would underflow `to_bits() - …`. Such a sample carries no
        // depth information, so skip it.
        if reconstructed_value == 0 {
            continue;
        }

        let sample_depth: u8 =
            source.depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

        if sample_depth > actual_depth {
            actual_depth = sample_depth;
        }

        if actual_depth == source.depth.to_bits() {
            break;
        }
    }

    Ok(actual_depth)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::{
        dsp::analysis::bit_depth::calculate_true_depth,
        model::{bit_depth::BitDepth, frequency::Frequency, stereo_signal::StereoSignal, MAX_16_BIT, MAX_8_BIT},
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
        let stereo = StereoSignal {
            left,
            right,
            sample_rate: Frequency::CdQuality,
            depth
        };
        let res = calculate_true_depth(&stereo).unwrap();

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
        let stereo = StereoSignal {
            left,
            right,
            sample_rate: Frequency::CdQuality,
            depth
        };
        let res = calculate_true_depth(&stereo).unwrap();

        assert_eq!(res, 8u8);
    }

    // Non-zero sample below 0.5 LSB: sample*factor truncates to 0.
    // Must contribute no depth (skip), never underflow `to_bits() - 32`.
    #[test]
    fn sub_lsb_sample_does_not_underflow() {
        let left: crate::model::Signal = [0.4 / MAX_16_BIT].iter().copied().collect();
        let right: crate::model::Signal = [0.0].iter().copied().collect();
        let depth = BitDepth::new(16).unwrap();
        let stereo = StereoSignal {
            left,
            right,
            sample_rate: Frequency::CdQuality,
            depth,
        };
        let res = calculate_true_depth(&stereo).unwrap();
        assert_eq!(res, 0u8);
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
        let stereo = StereoSignal {
            left,
            right,
            sample_rate: Frequency::CdQuality,
            depth
        };
        let res = calculate_true_depth(&stereo).unwrap();

        assert_eq!(res, 15u8);
    }
}
