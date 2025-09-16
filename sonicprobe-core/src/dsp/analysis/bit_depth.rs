use crate::model::{
    MAX_8_BIT, MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, bit_depth::BitDepth,
    sonicprobe_error::SonicProbeError, stereo_signal::StereoSignal,
};

/// Given a `stereo_signal`, loops over each sample and looks for the one with
/// the least amount of 0s to the left.
/// 
/// If the reported bit depth is `reported_bit_depth` and the actual bit depth,
/// calculated using the method stated before, is `actual_bit_depth`
/// 
/// Returns `Ok(min(reported_bit_depth, actual_bit_depth))` on success, 
/// otherwise returns an error.
/// 
/// This function has no side effects.
/// This function is declared as `#[inline]`
///
/// # Errors
/// 
/// Returns [`SonicProbeError`](crate::model::sonicprobe_error::SonicProbeError) 
/// if casting from u32 to u8 fails after calling the method 
/// `i32::trailing_zeros()` on the reconstructed sample
///
/// # Examples
/// 
/// ```
/// let left = [10, 20, 30, 111]
///     .iter()
///     .map(|val: &u8| f64::from(*val) / MAX_8_BIT)
///     .collect();
/// let right = [10, 20, 30, 111]
///     .iter()
///     .map(|val: &u8| f64::from(*val) / MAX_8_BIT)
///     .collect();
/// let depth = BitDepth::new(8).unwrap();
/// let stereo = StereoSignal {
///     left,
///     right,
///     sample_rate: Frequency::CdQuality,
///     depth
/// };
/// let res = calculate_true_depth(&stereo).unwrap();
/// ```
///
#[inline]
pub fn calculate_true_depth(stereo_signal: &StereoSignal) -> Result<u8, SonicProbeError> {
    let factor = match stereo_signal.depth {
        BitDepth::Legacy => MAX_8_BIT,
        BitDepth::CdStandard => MAX_16_BIT,
        BitDepth::Professional => MAX_24_BIT,
        BitDepth::StudioMaster => MAX_32_BIT,
    };

    let mut actual_bit_depth = 0u8;

    for &sample in stereo_signal.left.iter() {
        if sample == 0.0 {
            continue;
        }

        let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };

        let sample_depth: u8 =
            stereo_signal.depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

        if sample_depth > actual_bit_depth {
            actual_bit_depth = sample_depth;
        }

        if actual_bit_depth == stereo_signal.depth.to_bits() {
            break;
        }
    }

    for &sample in stereo_signal.right.iter() {
        if sample == 0.0 {
            continue;
        }

        let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };

        let sample_depth: u8 =
            stereo_signal.depth.to_bits() - u8::try_from(reconstructed_value.trailing_zeros())?;

        if sample_depth > actual_bit_depth {
            actual_bit_depth = sample_depth;
        }

        if actual_bit_depth == stereo_signal.depth.to_bits() {
            break;
        }
    }

    Ok(actual_bit_depth)
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
