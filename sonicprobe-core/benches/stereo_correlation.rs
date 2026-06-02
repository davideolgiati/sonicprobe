//! Benchmark + accuracy harness for `calculate_stereo_correlation`.
//!
//! Drives the normalized-correlation hot loop flagged with a per-iteration
//! bounds check on `right[i]` plus scalar-only serial accumulator chains.
//!
//! Input is a deterministic, realistic stereo pair (shared mid content + a
//! decorrelated side signal), so the correlation value and timing are stable
//! across runs and atomic refactors can be compared via criterion's baseline.
//!
//! `frozen_naive_correlation` is a verbatim copy of the ORIGINAL loop, kept as
//! the accuracy oracle. Every bench iteration asserts the production result
//! matches it within a tight epsilon.

use std::f64::consts::PI;
use std::hint::black_box;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use sonicprobe_core::analysis::stereo_correlation::calculate_stereo_correlation;
use sonicprobe_core::Signal;

const SAMPLE_RATE: f64 = 44_100.0;

/// Deterministic, music-like stereo pair in [-1.0, 1.0].
///
/// Builds a mono "mid" (harmonics + tremolo) plus a lower-level, decorrelated
/// "side", then `L = mid + side`, `R = mid - side`. This yields a realistic
/// inter-channel correlation strictly between 0 and 1 — what the loop sees on
/// real stereo music, not a degenerate identical/orthogonal pair.
fn realistic_stereo(samples: usize) -> (Signal, Signal) {
    const PARTIALS: [(f64, f64); 5] = [
        (110.0, 0.50),
        (220.0, 0.30),
        (330.0, 0.15),
        (440.0, 0.10),
        (1760.0, 0.05),
    ];

    let mut rng = StdRng::seed_from_u64(0x57E2_E0_C0FF_EE);
    let mut left = Vec::with_capacity(samples);
    let mut right = Vec::with_capacity(samples);

    for i in 0..samples {
        let t = i as f64 / SAMPLE_RATE;
        let envelope = 0.6 + 0.4 * (2.0 * PI * 3.0 * t).sin();

        let mut mid = 0.0;
        for (freq, amp) in PARTIALS {
            mid += amp * (2.0 * PI * freq * t).sin();
        }
        mid *= envelope;

        // Decorrelated stereo side: a different partial + noise, ~30% level.
        let side = 0.3 * (2.0 * PI * 277.0 * t).sin() + (rng.random::<f64>() - 0.5) * 0.05;

        left.push((mid + side).clamp(-1.0, 1.0));
        right.push((mid - side).clamp(-1.0, 1.0));
    }

    (Arc::from(left), Arc::from(right))
}

/// FROZEN baseline: verbatim original loop, the accuracy oracle. Do NOT optimize.
fn frozen_naive_correlation(left: &[f64], right: &[f64]) -> f64 {
    let mut left_square_sum: f64 = 0.0;
    let mut right_square_sum: f64 = 0.0;
    let mut correlation: f64 = 0.0;

    for i in 0..left.len() {
        correlation += left[i] * right[i];
        left_square_sum += left[i].powi(2);
        right_square_sum += right[i].powi(2);
    }

    correlation / (left_square_sum * right_square_sum).sqrt()
}

fn bench_correlation(c: &mut Criterion) {
    let sizes = [1_000_000usize, 4_200_000, 8_400_000];

    let mut group = c.benchmark_group("stereo_correlation");

    for &n in &sizes {
        let (left, right) = realistic_stereo(n);

        // Accuracy gate vs the frozen oracle. Lane-splitting reorders adds and
        // FP is non-associative, so results match to epsilon, not bit-for-bit.
        let produced = calculate_stereo_correlation(&left, &right);
        let oracle = frozen_naive_correlation(&left, &right);
        let abs_err = (produced - oracle).abs();
        let rel_err = abs_err / oracle.abs();
        println!(
            "[accuracy] n={n:>9}  produced={produced:.17}  oracle={oracle:.17}  \
             abs_err={abs_err:.3e}  rel_err={rel_err:.3e}"
        );
        assert!(
            abs_err < 1e-12,
            "correlation accuracy regression at n={n}: produced={produced}, \
             oracle={oracle}, abs_err={abs_err}"
        );

        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(n),
            &(left, right),
            |b, (l, r)| {
                b.iter(|| calculate_stereo_correlation(black_box(l), black_box(r)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_correlation);
criterion_main!(benches);