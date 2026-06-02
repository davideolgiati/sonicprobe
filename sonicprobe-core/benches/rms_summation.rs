//! Benchmark + accuracy harness for the RMS summation hot path.
//!
//! Drives `compute_root_mean_square`, which inlines `map_sum_lossless`
//! (the Kahan compensated-summation loop flagged as a SERIAL_CHAIN).
//!
//! The input is a deterministic, music-like signal so numbers are stable
//! across runs and refactors can be compared via criterion's saved baseline.
//!
//! `frozen_kahan_rms` is a verbatim copy of the ORIGINAL Kahan implementation.
//! Every bench iteration asserts the production result matches it within a
//! tight epsilon — so an ILP refactor that quietly loses accuracy fails here.

use std::f64::consts::PI;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use sonicprobe_core::analysis::root_mean_square::compute_root_mean_square;

/// Sample rate used to place the partials at musically realistic frequencies.
const SAMPLE_RATE: f64 = 44_100.0;

/// Generate a deterministic, music-like mono signal in [-1.0, 1.0].
///
/// Models what a real decoded FLAC channel looks like to the summation loop:
/// a fundamental plus a few harmonics, a slow tremolo amplitude envelope, and
/// a low dither/noise floor. Deterministic (fixed seed) so the RMS value and
/// timing are reproducible.
fn realistic_signal(samples: usize) -> Vec<f64> {
    // Bass-ish fundamental at 110 Hz (A2) + harmonics, decreasing amplitude.
    const PARTIALS: [(f64, f64); 5] = [
        (110.0, 0.50),
        (220.0, 0.30),
        (330.0, 0.15),
        (440.0, 0.10),
        (1760.0, 0.05),
    ];

    let mut rng = StdRng::seed_from_u64(0x5081_C_C0FF_EE);
    let mut out = Vec::with_capacity(samples);

    for i in 0..samples {
        let t = i as f64 / SAMPLE_RATE;

        // ~3 Hz tremolo: amplitude breathes between 0.2 and 1.0.
        let envelope = 0.6 + 0.4 * (2.0 * PI * 3.0 * t).sin();

        let mut sample = 0.0;
        for (freq, amp) in PARTIALS {
            sample += amp * (2.0 * PI * freq * t).sin();
        }
        sample *= envelope;

        // ~ -54 dB noise floor.
        sample += (rng.random::<f64>() - 0.5) * 0.002;

        out.push(sample.clamp(-1.0, 1.0));
    }

    out
}

/// FROZEN baseline: original `map_sum_lossless` + RMS, kept verbatim as the
/// accuracy oracle. Do NOT optimize this copy — it is the reference.
fn frozen_kahan_rms(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;

    for &current_value in values {
        let mapped_value = current_value.powi(2);
        compensation =
            ((sum + (mapped_value - compensation)) - sum) - (mapped_value - compensation);
        sum += mapped_value - compensation;
    }

    (sum / values.len() as f64).sqrt()
}

fn bench_rms(c: &mut Criterion) {
    // CD-rate channel lengths: ~23s, ~95s, ~3.2 min.
    let sizes = [1_000_000usize, 4_200_000, 8_400_000];

    let mut group = c.benchmark_group("rms_summation");

    for &n in &sizes {
        let signal = realistic_signal(n);

        // Accuracy gate: production must track the frozen Kahan oracle.
        // Parallel-lane summation reorders adds and FP is non-associative, so
        // results match to a tight epsilon rather than bit-for-bit.
        let produced = compute_root_mean_square(&signal).expect("non-empty signal");
        let oracle = frozen_kahan_rms(&signal);
        let abs_err = (produced - oracle).abs();
        let rel_err = abs_err / oracle.abs();
        println!(
            "[accuracy] n={n:>9}  produced={produced:.17}  oracle={oracle:.17}  \
             abs_err={abs_err:.3e}  rel_err={rel_err:.3e}"
        );
        assert!(
            abs_err < 1e-12,
            "RMS accuracy regression at n={n}: produced={produced}, oracle={oracle}, \
             abs_err={abs_err}"
        );

        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &signal, |b, sig| {
            b.iter(|| compute_root_mean_square(black_box(sig)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_rms);
criterion_main!(benches);
