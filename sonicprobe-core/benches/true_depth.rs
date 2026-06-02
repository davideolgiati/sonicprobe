//! Benchmark + accuracy harness for `calculate_true_depth`.
//!
//! Drives the per-sample reconstruction loop flagged with a redundant `.trunc()`
//! that lowers to a libm `call` before the `cvttsd2si` (which already truncates
//! toward zero) — one wasted function call per non-zero sample.
//!
//! The function early-`break`s once `actual_depth` reaches the container depth.
//! Realistic full-scale music hits full depth on the first loud sample, so to
//! measure the per-sample hot path we feed a WORST-CASE full-scan signal: every
//! 16-bit code is forced EVEN (LSB = 0), so `sample_depth` tops out at 15, never
//! equals 16, and the loop scans every sample — exactly the path whose per-sample
//! cost we are optimizing.
//!
//! `frozen_true_depth` is a verbatim copy of the ORIGINAL function, kept as the
//! accuracy oracle. Dropping `.trunc()` is bit-identical (cvttsd2si truncates the
//! same way), so the gate asserts exact equality.

use std::f64::consts::PI;
use std::hint::black_box;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use sonicprobe_core::analysis::bit_depth::calculate_true_depth;
use sonicprobe_core::bit_depth::BitDepth;
use sonicprobe_core::frequency::Frequency;
use sonicprobe_core::stereo_signal::StereoSignal;
use sonicprobe_core::{Signal, MAX_16_BIT};

const SAMPLE_RATE: f64 = 44_100.0;

/// Deterministic, music-like signal quantized to EVEN 16-bit codes.
///
/// Forces a full scan: even codes mean the LSB is always 0, so the detected
/// depth caps at 15 and the early-`break` never fires.
fn full_scan_channel(samples: usize, phase: f64) -> Signal {
    const PARTIALS: [(f64, f64); 5] = [
        (110.0, 0.50),
        (220.0, 0.30),
        (330.0, 0.15),
        (440.0, 0.10),
        (1760.0, 0.05),
    ];

    let mut out = Vec::with_capacity(samples);
    for i in 0..samples {
        let t = i as f64 / SAMPLE_RATE;
        let envelope = 0.6 + 0.4 * (2.0 * PI * 3.0 * t + phase).sin();

        let mut v = 0.0;
        for (freq, amp) in PARTIALS {
            v += amp * (2.0 * PI * freq * t + phase).sin();
        }
        v *= envelope;

        // Quantize to a 16-bit code, then force it EVEN and non-zero.
        let mut code = (v.clamp(-0.999, 0.999) * MAX_16_BIT) as i32;
        code &= !1;
        if code == 0 {
            code = 2;
        }
        out.push(f64::from(code) / MAX_16_BIT);
    }
    Arc::from(out)
}

fn make_signal(samples: usize) -> StereoSignal {
    StereoSignal {
        left: full_scan_channel(samples, 0.0),
        right: full_scan_channel(samples, 1.3),
        sample_rate: Frequency::CdQuality,
        depth: BitDepth::new(16).expect("16 is a valid depth"),
    }
}

/// FROZEN baseline: verbatim original function, the accuracy oracle. Do NOT optimize.
fn frozen_true_depth(source: &StereoSignal) -> u8 {
    let factor = MAX_16_BIT; // benchmark fixes depth at 16-bit

    let mut actual_depth = 0u8;

    for &sample in source.left.iter() {
        if sample == 0.0 {
            continue;
        }
        let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };
        if reconstructed_value == 0 {
            continue;
        }
        let sample_depth: u8 =
            source.depth.to_bits() - reconstructed_value.trailing_zeros() as u8;
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
        let reconstructed_value: i32 = unsafe { (sample * factor).trunc().to_int_unchecked() };
        if reconstructed_value == 0 {
            continue;
        }
        let sample_depth: u8 =
            source.depth.to_bits() - reconstructed_value.trailing_zeros() as u8;
        if sample_depth > actual_depth {
            actual_depth = sample_depth;
        }
        if actual_depth == source.depth.to_bits() {
            break;
        }
    }

    actual_depth
}

fn bench_true_depth(c: &mut Criterion) {
    let sizes = [1_000_000usize, 4_200_000, 8_400_000];

    let mut group = c.benchmark_group("true_depth");

    for &n in &sizes {
        let signal = make_signal(n);

        let produced = calculate_true_depth(&signal).expect("valid signal");
        let oracle = frozen_true_depth(&signal);
        println!("[accuracy] n={n:>9}  produced={produced}  oracle={oracle}");
        assert_eq!(
            produced, oracle,
            "true_depth accuracy regression at n={n}: produced={produced}, oracle={oracle}"
        );
        // Sanity: the worst-case signal must actually force a full scan.
        assert_eq!(produced, 15, "expected even-code signal to cap depth at 15");

        group.throughput(Throughput::Elements(2 * n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &signal, |b, s| {
            b.iter(|| calculate_true_depth(black_box(s)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_true_depth);
criterion_main!(benches);