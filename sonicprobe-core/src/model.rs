use std::sync::Arc;

pub mod audio_file;
pub mod bit_depth;
pub mod builders;
pub mod channel;
pub mod frequency;
pub mod sonicprobe_error;
pub mod stereo_signal;
pub mod decibel;
pub mod dynamic_range;

pub type Samples = f64;
pub type Signal = Arc<[Samples]>;
pub type Milliseconds = usize;

pub const MAX_8_BIT: f64 = 127.0;
pub const MAX_16_BIT: f64 = 32767.0;
pub const MAX_24_BIT: f64 = 8_388_607.0;
pub const MAX_32_BIT: f64 = 2_147_483_647.0;

#[allow(clippy::unreadable_literal)]
pub const SD_COEFFS: [[f64; 12]; 4] = [
    [0.0017089843750, 0.0109863281250, -0.0196533203125, 0.0332031250000, -0.0594482421875, 0.1373291015625, 0.9721679687500, -0.1022949218750, 0.0476074218750, -0.0266113281250, 0.0148925781250, -0.0083007812500],
    [-0.0291748046875, 0.0292968750000, -0.0517578125000, 0.0891113281250, -0.1665039062500, 0.4650878906250, 0.7797851562500, -0.2003173828125, 0.1015625000000, -0.0582275390625, 0.0330810546875, -0.0189208984375],
    [-0.0189208984375, 0.0330810546875, -0.0582275390625, 0.1015625000000, -0.2003173828125, 0.7797851562500, 0.4650878906250, -0.1665039062500, 0.0891113281250, -0.0517578125000, 0.0292968750000, -0.0291748046875],
    [-0.0083007812500, 0.0148925781250, -0.0266113281250, 0.0476074218750, -0.1022949218750, 0.9721679687500, 0.1373291015625, -0.0594482421875, 0.0332031250000, -0.0196533203125, 0.0109863281250, 0.0017089843750]
];

#[allow(clippy::unreadable_literal)]
pub const HD_COEFFS: [[f64; 12]; 2] = [
    [-0.0005187988281, -0.0053405761719, -0.0203857421875, -0.0561523437500, -0.1370849609375, -0.4033050537109, 1.2660980224609, 0.2207641601562, 0.0880126953125, 0.0347442626953, 0.0110626220703, 0.0021057128906],
    [0.0021057128906, 0.0110626220703, 0.0347442626953, 0.0880126953125, 0.2207641601562, 1.2660980224609, -0.4033050537109, -0.1370849609375, -0.0561523437500, -0.0203857421875, -0.0053405761719, -0.0005187988281],
];

/// Tap-major (transposed) view of a polyphase coefficient table:
/// `out[tap][phase] == src[phase][tap]`. Pure reindex — bit-identical values.
/// Used by the broadcast-FMA upsampling kernel, where one vector load pulls all
/// `PHASES` coefficients for a given tap so the phases compute in parallel lanes.
const fn transpose_coeffs<const PHASES: usize>(
    src: &[[f64; 12]; PHASES],
) -> [[f64; PHASES]; 12] {
    let mut out = [[0.0f64; PHASES]; 12];
    let mut tap = 0;
    while tap < 12 {
        let mut phase = 0;
        while phase < PHASES {
            out[tap][phase] = src[phase][tap];
            phase += 1;
        }
        tap += 1;
    }
    out
}

/// Tap-major SD coefficients (4 phases per tap) for the ×4 broadcast-FMA kernel.
pub const SD_COEFFS_T: [[f64; 4]; 12] = transpose_coeffs(&SD_COEFFS);

/// Tap-major HD coefficients (2 phases per tap) for the ×2 broadcast-FMA kernel.
pub const HD_COEFFS_T: [[f64; 2]; 12] = transpose_coeffs(&HD_COEFFS);

#[cfg(test)]
mod coeff_transpose_tests {
    use super::{HD_COEFFS, HD_COEFFS_T, SD_COEFFS, SD_COEFFS_T};

    #[test]
    fn sd_transpose_is_exact_reindex() {
        for tap in 0..12 {
            for phase in 0..4 {
                assert_eq!(SD_COEFFS_T[tap][phase], SD_COEFFS[phase][tap]);
            }
        }
    }

    #[test]
    fn hd_transpose_is_exact_reindex() {
        for tap in 0..12 {
            for phase in 0..2 {
                assert_eq!(HD_COEFFS_T[tap][phase], HD_COEFFS[phase][tap]);
            }
        }
    }
}
