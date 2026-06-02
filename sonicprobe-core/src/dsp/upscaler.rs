use crate::model::{
    HD_COEFFS_T, SD_COEFFS_T, frequency::Frequency, sonicprobe_error::SonicProbeError,
};

use std::f64;

/// Filter overhang. A 12-tap window starting up to `TAPS - 1` samples before
/// (or after) the signal still overlaps it, so padding the input by this much
/// each end — treating samples outside the signal as silence — lets every
/// input sample's interpolated true-peak reach the meter, including the
/// boundary intersample peaks the old `len - 13` range silently dropped.
const PAD: isize = 11;

/// Number of fully in-bounds 12-sample windows `source[i..i + 12]`, `i` in
/// `0..interior_count`. The head/tail overhang windows are handled separately.
#[inline]
fn interior_count(len: usize) -> usize {
    len.saturating_sub(11)
}

/// Polyphase upsampling fused with the only thing the caller reduces it to:
/// `(peak |sample|, clipping-sample count)`. No oversampled sample is
/// materialized — each input window produces all phase outputs in parallel
/// vector lanes (tap-major coefficients, broadcast input), folded straight into
/// the peak / clip accumulators. The signal is conceptually zero-padded by
/// [`PAD`] each end so boundary true-peaks are not missed.
pub fn upsample_reduce(
    source: &[f64],
    source_sample_rate: Frequency,
) -> Result<(f64, u64), SonicProbeError> {
    match source_sample_rate {
        Frequency::CdQuality | Frequency::ProAudio => Ok(reduce_sd(source)),
        Frequency::HiResDouble | Frequency::DvdAudio => Ok(reduce_hd(source)),
        Frequency::StudioMaster | Frequency::UltraHiRes => Err(SonicProbeError {
            location: format!("{}:{}", file!(), line!()),
            message: "upscaling for 176,400Hz and 192,000Hz not implemented".to_owned(),
        }),
    }
}

/// Folds one oversampled output into the running peak/clip accumulators,
/// matching the original `|o| >= 1.0` clip test (≡ `o >= 1.0 || o <= -1.0`).
#[inline]
fn fold(output: f64, peak: &mut f64, clip: &mut u64) {
    let magnitude = output.abs();
    if magnitude >= 1.0 {
        *clip += 1;
    }
    if magnitude > *peak {
        *peak = magnitude;
    }
}

/// ×4 (SD) reduction over the zero-padded signal: scalar zero-fill for the head
/// and tail overhang windows, the fast in-bounds kernel for the interior.
fn reduce_sd(source: &[f64]) -> (f64, u64) {
    let len = source.len() as isize;
    let mut peak = f64::MIN;
    let mut clip = 0u64;

    // Too short for a single in-bounds window: every window overhangs an edge.
    if len < 12 {
        for start in -PAD..len {
            fold_padded_window_sd(source, start, &mut peak, &mut clip);
        }
        return (peak, clip);
    }

    // Head overhang: windows starting before sample 0 (disjoint from interior).
    for start in -PAD..0 {
        fold_padded_window_sd(source, start, &mut peak, &mut clip);
    }
    // Interior: every fully in-bounds window via the fast kernel.
    let (interior_peak, interior_clip) = interior_sd(source);
    if interior_peak > peak {
        peak = interior_peak;
    }
    clip += interior_clip;
    // Tail overhang: windows running past the last sample.
    for start in (len - PAD)..len {
        fold_padded_window_sd(source, start, &mut peak, &mut clip);
    }

    (peak, clip)
}

/// ×2 (HD) reduction over the zero-padded signal (see [`reduce_sd`]).
fn reduce_hd(source: &[f64]) -> (f64, u64) {
    let len = source.len() as isize;
    let mut peak = f64::MIN;
    let mut clip = 0u64;

    if len < 12 {
        for start in -PAD..len {
            fold_padded_window_hd(source, start, &mut peak, &mut clip);
        }
        return (peak, clip);
    }

    for start in -PAD..0 {
        fold_padded_window_hd(source, start, &mut peak, &mut clip);
    }
    let (interior_peak, interior_clip) = interior_hd(source);
    if interior_peak > peak {
        peak = interior_peak;
    }
    clip += interior_clip;
    for start in (len - PAD)..len {
        fold_padded_window_hd(source, start, &mut peak, &mut clip);
    }

    (peak, clip)
}

/// Interior ×4 dispatch: AVX+FMA broadcast kernel when available, scalar
/// otherwise.
///
/// (An AVX-512 variant packing 2 windows/iter into a `__m512d` was tried and
/// measured ~13 % SLOWER here — the per-tap insert to build the dual-window
/// sample vector plus AVX-512 frequency licensing outweigh the halved iteration
/// count; the kernel is sample-broadcast bound, not vector-width bound.)
fn interior_sd(source: &[f64]) -> (f64, u64) {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx") && std::is_x86_feature_detected!("fma") {
            // SAFETY: guarded by runtime detection of avx + fma.
            return unsafe { reduce_sd_avx(source) };
        }
    }
    reduce_sd_scalar(source)
}

/// Interior ×2 dispatch: AVX+FMA broadcast kernel when available, scalar otherwise.
fn interior_hd(source: &[f64]) -> (f64, u64) {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx") && std::is_x86_feature_detected!("fma") {
            // SAFETY: guarded by runtime detection of avx + fma.
            return unsafe { reduce_hd_avx(source) };
        }
    }
    reduce_hd_scalar(source)
}

/// Folds the 4 phase outputs of one window that overhangs a signal edge,
/// treating samples outside `source` as silence (zero).
#[inline]
fn fold_padded_window_sd(source: &[f64], start: isize, peak: &mut f64, clip: &mut u64) {
    let len = source.len() as isize;
    let mut acc = [0.0f64; 4];
    for tap in 0..12 {
        let index = start + tap;
        if index < 0 || index >= len {
            continue;
        }
        let sample = source[index as usize];
        let coefficients = &SD_COEFFS_T[tap as usize];
        acc[0] += sample * coefficients[0];
        acc[1] += sample * coefficients[1];
        acc[2] += sample * coefficients[2];
        acc[3] += sample * coefficients[3];
    }
    for output in acc {
        fold(output, peak, clip);
    }
}

/// ×2 counterpart of [`fold_padded_window_sd`].
#[inline]
fn fold_padded_window_hd(source: &[f64], start: isize, peak: &mut f64, clip: &mut u64) {
    let len = source.len() as isize;
    let mut acc = [0.0f64; 2];
    for tap in 0..12 {
        let index = start + tap;
        if index < 0 || index >= len {
            continue;
        }
        let sample = source[index as usize];
        let coefficients = &HD_COEFFS_T[tap as usize];
        acc[0] += sample * coefficients[0];
        acc[1] += sample * coefficients[1];
    }
    for output in acc {
        fold(output, peak, clip);
    }
}

/// Portable scalar interior ×4 reducer (fallback for the AVX kernel). Four
/// independent phase accumulators per window over the tap-major coefficients.
fn reduce_sd_scalar(source: &[f64]) -> (f64, u64) {
    let mut peak = f64::MIN;
    let mut clip = 0u64;

    for start in 0..interior_count(source.len()) {
        let Ok(window) = <&[f64; 12]>::try_from(&source[start..start + 12]) else {
            continue;
        };
        let mut acc = [0.0f64; 4];
        for (sample, coefficients) in window.iter().zip(&SD_COEFFS_T) {
            acc[0] += sample * coefficients[0];
            acc[1] += sample * coefficients[1];
            acc[2] += sample * coefficients[2];
            acc[3] += sample * coefficients[3];
        }
        for output in acc {
            fold(output, &mut peak, &mut clip);
        }
    }

    (peak, clip)
}

/// Portable scalar interior ×2 reducer (fallback for the AVX kernel).
fn reduce_hd_scalar(source: &[f64]) -> (f64, u64) {
    let mut peak = f64::MIN;
    let mut clip = 0u64;

    for start in 0..interior_count(source.len()) {
        let Ok(window) = <&[f64; 12]>::try_from(&source[start..start + 12]) else {
            continue;
        };
        let mut acc = [0.0f64; 2];
        for (sample, coefficients) in window.iter().zip(&HD_COEFFS_T) {
            acc[0] += sample * coefficients[0];
            acc[1] += sample * coefficients[1];
        }
        for output in acc {
            fold(output, &mut peak, &mut clip);
        }
    }

    (peak, clip)
}

/// ×4 broadcast-FMA kernel. The four phases live in the four lanes of one
/// `__m256d`: for each of the 12 taps, broadcast the input sample and FMA
/// against the tap's 4 phase coefficients (one contiguous load from the
/// tap-major table). No per-phase horizontal reduction in the loop — peak and
/// clip are reduced in vector lanes, collapsed once at the end.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,fma")]
#[allow(clippy::wildcard_imports)]
unsafe fn reduce_sd_avx(source: &[f64]) -> (f64, u64) {
    use core::arch::x86_64::*;

    let count = interior_count(source.len());
    let base = source.as_ptr();
    let abs_mask = _mm256_set1_pd(-0.0); // andnot clears the sign bit -> |x|
    let one = _mm256_set1_pd(1.0);
    let mut peak_v = _mm256_set1_pd(f64::MIN);
    let mut clip = 0u64;

    for start in 0..count {
        let window = base.add(start);
        // Two independent FMA chains (even/odd taps) to break the serial
        // dependency on a single accumulator.
        let mut acc0 = _mm256_setzero_pd();
        let mut acc1 = _mm256_setzero_pd();
        let mut tap = 0;
        while tap < 12 {
            let s0 = _mm256_set1_pd(*window.add(tap));
            acc0 = _mm256_fmadd_pd(s0, _mm256_loadu_pd(SD_COEFFS_T[tap].as_ptr()), acc0);
            let s1 = _mm256_set1_pd(*window.add(tap + 1));
            acc1 = _mm256_fmadd_pd(s1, _mm256_loadu_pd(SD_COEFFS_T[tap + 1].as_ptr()), acc1);
            tap += 2;
        }
        let acc = _mm256_add_pd(acc0, acc1);
        let magnitude = _mm256_andnot_pd(abs_mask, acc);
        peak_v = _mm256_max_pd(peak_v, magnitude);
        let clip_mask = _mm256_cmp_pd::<_CMP_GE_OQ>(magnitude, one);
        clip += u64::from((_mm256_movemask_pd(clip_mask) as u32).count_ones());
    }

    // Horizontal max of the 4 lanes.
    let hi = _mm256_extractf128_pd::<1>(peak_v);
    let lo = _mm256_castpd256_pd128(peak_v);
    let pair = _mm_max_pd(lo, hi);
    let peak = _mm_cvtsd_f64(_mm_max_sd(pair, _mm_unpackhi_pd(pair, pair)));
    (peak, clip)
}

/// ×2 broadcast-FMA kernel — two phases in the two lanes of one `__m128d`.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx,fma")]
#[allow(clippy::wildcard_imports)]
unsafe fn reduce_hd_avx(source: &[f64]) -> (f64, u64) {
    use core::arch::x86_64::*;

    let count = interior_count(source.len());
    let base = source.as_ptr();
    let abs_mask = _mm_set1_pd(-0.0);
    let one = _mm_set1_pd(1.0);
    let mut peak_v = _mm_set1_pd(f64::MIN);
    let mut clip = 0u64;

    for start in 0..count {
        let window = base.add(start);
        let mut acc = _mm_setzero_pd();
        for tap in 0..12 {
            let sample = _mm_set1_pd(*window.add(tap));
            let coefficients = _mm_loadu_pd(HD_COEFFS_T[tap].as_ptr());
            acc = _mm_fmadd_pd(sample, coefficients, acc);
        }
        let magnitude = _mm_andnot_pd(abs_mask, acc);
        peak_v = _mm_max_pd(peak_v, magnitude);
        let clip_mask = _mm_cmp_pd::<_CMP_GE_OQ>(magnitude, one);
        clip += u64::from((_mm_movemask_pd(clip_mask) as u32).count_ones());
    }

    let peak = _mm_cvtsd_f64(_mm_max_sd(peak_v, _mm_unpackhi_pd(peak_v, peak_v)));
    (peak, clip)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::dsp::upsample_chain;
    use crate::model::{frequency::Frequency, Signal};
    use std::f64::consts::PI;
    use std::sync::Arc;

    fn sine(n: usize, amp: f64, freq: f64) -> Signal {
        let v: Vec<f64> = (0..n)
            .map(|i| amp * (2.0 * PI * freq * i as f64 / 44_100.0).sin())
            .collect();
        Arc::from(v)
    }

    // Impl-independent: silence in → no clipping, and the peak collapses to
    // zero magnitude (Decibel::new(0.0) == -inf). True for any upsampler.
    #[test]
    fn silence_has_no_clipping_and_zero_peak() {
        let signal: Signal = Arc::from(vec![0.0f64; 2_000]);
        let (peak, clip) = upsample_chain(&signal, Frequency::CdQuality).unwrap();
        assert_eq!(clip, 0);
        assert!(peak.get_value().is_infinite() && peak.get_value() < 0.0);
    }

    // Impl-independent: the filter is linear, so doubling the input lifts the
    // peak by ~6.0206 dB (20*log10(2)), regardless of how the dot is computed.
    #[test]
    fn doubling_input_adds_six_db() {
        let quiet = sine(4_000, 0.3, 1_000.0);
        let loud = sine(4_000, 0.6, 1_000.0);
        let (q, _) = upsample_chain(&quiet, Frequency::CdQuality).unwrap();
        let (l, _) = upsample_chain(&loud, Frequency::CdQuality).unwrap();
        let delta = l.get_value() - q.get_value();
        assert!((delta - 6.020_6).abs() < 0.01, "delta={delta}");
    }

    // Impl-independent: a constant over-unity DC signal must drive the
    // interpolated output above 1.0 → some clipping detected.
    #[test]
    fn over_unity_dc_clips() {
        let signal: Signal = Arc::from(vec![1.5f64; 2_000]);
        let (_peak, clip) = upsample_chain(&signal, Frequency::CdQuality).unwrap();
        assert!(clip > 0, "expected clipping, got {clip}");
    }

    // HD path (x2) must also run without panic and detect no clipping on a
    // safely-scaled sine.
    #[test]
    fn hd_path_runs() {
        let signal = sine(4_000, 0.5, 2_000.0);
        let (_peak, clip) = upsample_chain(&signal, Frequency::HiResDouble).unwrap();
        assert_eq!(clip, 0);
    }

    // Boundary coverage (BS.1770): a full-scale-plus spike on the very last
    // sample must reach the meter. The old `len - 13` window range dropped the
    // tail, so this clipping peak went unseen.
    #[test]
    fn spike_at_last_sample_is_detected() {
        let mut data = vec![0.0f64; 2_000];
        data[1_999] = 5.0;
        let signal: Signal = Arc::from(data);
        let (peak, clip) = upsample_chain(&signal, Frequency::CdQuality).unwrap();
        assert!(
            clip > 0 && peak.get_value() > 0.0,
            "last-sample true-peak must be detected: clip={clip}, peak_db={}",
            peak.get_value()
        );
    }

    // Symmetric: a spike on the FIRST sample must also reach the meter.
    #[test]
    fn spike_at_first_sample_is_detected() {
        let mut data = vec![0.0f64; 2_000];
        data[0] = 5.0;
        let signal: Signal = Arc::from(data);
        let (_peak, clip) = upsample_chain(&signal, Frequency::CdQuality).unwrap();
        assert!(clip > 0, "first-sample true-peak must be detected: clip={clip}");
    }
}
