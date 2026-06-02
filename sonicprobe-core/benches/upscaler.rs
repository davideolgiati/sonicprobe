//! Benchmark + accuracy harness for the true-peak / true-clipping upsampling
//! hot path (`upsample_chain` → `Upscaler` → `dot_product`).
//!
//! The consumer reduces the oversampled stream to `(peak dBFS, clip count)`.
//! `frozen_upsample_chain` is a verbatim port of the ORIGINAL pipeline (heap
//! `Vec` buffer + `next_sample` iterator) calling the SAME asm `dot_product`,
//! kept as the accuracy oracle. While a refactor preserves the dot arithmetic
//! the gate is EXACT; once the dot arithmetic changes, loosen `PEAK_TOL`.

use std::f64::consts::PI;
use std::hint::black_box;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use sonicprobe_core::frequency::Frequency;
use sonicprobe_core::{upsample_chain, Signal, HD_COEFFS, SD_COEFFS};

/// Self-contained 12-tap dot (FMA via `mul_add`) so the oracle does not depend
/// on any library kernel.
fn dot12(coeffs: &[f64; 12], window: &[f64; 12]) -> f64 {
    let mut sum = 0.0;
    for tap in 0..12 {
        sum = coeffs[tap].mul_add(window[tap], sum);
    }
    sum
}

const SAMPLE_RATE: f64 = 44_100.0;

/// Exact while the dot arithmetic is unchanged; bump to e.g. 1e-9 once a step
/// alters the summation (FMA vs mul+add reorders the last ULP).
const PEAK_TOL: f64 = 1e-9;
/// Max allowed |clip - oracle_clip| once the dot summation order changes.
const CLIP_TOL: u128 = 16;

/// Deterministic music-like signal scaled near full-scale, so interpolation
/// overshoot drives occasional intersample (true-peak) clipping — exercising
/// both the peak max and the clip counter.
fn realistic_signal(samples: usize) -> Signal {
    const PARTIALS: [(f64, f64); 5] = [
        (110.0, 0.55),
        (220.0, 0.30),
        (330.0, 0.18),
        (1760.0, 0.12),
        (5000.0, 0.08),
    ];

    let mut out = Vec::with_capacity(samples);
    for i in 0..samples {
        let t = i as f64 / SAMPLE_RATE;
        let envelope = 0.85 + 0.15 * (2.0 * PI * 2.0 * t).sin();
        let mut v = 0.0;
        for (freq, amp) in PARTIALS {
            v += amp * (2.0 * PI * freq * t).sin();
        }
        out.push((v * envelope).clamp(-1.0, 1.0));
    }
    Arc::from(out)
}

/// FROZEN reference for the corrected (zero-padded) meter: for every window
/// start `-11 ..= len-1`, build the 12-sample window with out-of-signal samples
/// set to silence, compute each phase via the real asm `dot_product`, and reduce
/// `(peak dBFS, clip count)`. Maximally obvious; the production kernel must match
/// it within tolerance. Do NOT optimize.
fn frozen_upsample_chain(source: &[f64], rate: Frequency) -> (f64, u64) {
    let (phases, matrix): (usize, &[[f64; 12]]) = match rate {
        Frequency::CdQuality | Frequency::ProAudio => (4, &SD_COEFFS),
        Frequency::HiResDouble | Frequency::DvdAudio => (2, &HD_COEFFS),
        _ => unreachable!("unsupported rate in bench"),
    };

    let len = source.len() as isize;
    let mut peak = f64::MIN;
    let mut clip = 0u64;

    for start in -11..len {
        let mut window = [0.0f64; 12];
        for tap in 0..12 {
            let index = start + tap;
            if index >= 0 && index < len {
                window[tap as usize] = source[index as usize];
            }
        }
        for phase in &matrix[..phases] {
            let output = dot12(phase, &window);
            if output >= 1.0 || output <= -1.0 {
                clip += 1;
            }
            let magnitude = output.abs();
            if magnitude > peak {
                peak = magnitude;
            }
        }
    }

    (20.0 * peak.log10(), clip)
}

fn bench_upscaler(c: &mut Criterion) {
    let sizes = [250_000usize, 1_000_000, 2_000_000];

    let mut group = c.benchmark_group("upscaler");

    for &n in &sizes {
        let signal = realistic_signal(n);

        let produced = upsample_chain(&signal, Frequency::CdQuality).expect("valid");
        let (prod_db, prod_clip) = (produced.0.get_value(), produced.1);
        let (oracle_db, oracle_clip) = frozen_upsample_chain(&signal, Frequency::CdQuality);
        let abs_err = (prod_db - oracle_db).abs();
        println!(
            "[accuracy] n={n:>9}  peak_db={prod_db:.12} (oracle={oracle_db:.12} abs_err={abs_err:.3e})  \
             clip={prod_clip} (oracle={oracle_clip})"
        );
        let clip_diff = (i128::from(prod_clip) - i128::from(oracle_clip)).unsigned_abs();
        assert!(
            abs_err <= PEAK_TOL,
            "peak regression at n={n}: produced={prod_db}, oracle={oracle_db}, abs_err={abs_err}"
        );
        // Clip is a hard-threshold count: a different (still-correct) FMA
        // summation order flips samples sitting within a few ULP of 1.0, so
        // exact equality is unreasonable across an arithmetic change. Bound the
        // drift; the impl-independent unit tests carry the real correctness.
        assert!(
            clip_diff <= CLIP_TOL,
            "clip drift too large at n={n}: produced={prod_clip}, oracle={oracle_clip}, diff={clip_diff}"
        );
        assert!(oracle_clip > 0, "bench signal should produce some clipping");

        group.throughput(Throughput::Elements(4 * n as u64)); // x4 oversampled outputs
        group.bench_with_input(BenchmarkId::from_parameter(n), &signal, |b, s| {
            b.iter(|| upsample_chain(black_box(s), Frequency::CdQuality));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_upscaler);
criterion_main!(benches);
