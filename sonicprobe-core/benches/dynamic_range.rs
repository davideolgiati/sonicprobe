//! Benchmark for the `DynamicRangeMeter` per-sample hot path (`push_sample`)
//! plus the final `get_dr_value`.
//!
//! The current implementation buffers every sample into a 3-second `Vec<f64>`
//! and batch-computes RMS per chunk; the accumulator rewrite should drop that
//! buffer entirely. This bench measures the full feed-and-read cost over a
//! realistic, amplitude-varying signal so the two approaches compare directly.
//!
//! The printed DR value is the cross-refactor accuracy check: it must stay
//! stable (to a tight tolerance) before vs after the rewrite.

use std::f64::consts::PI;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use sonicprobe_core::analysis::dynamic_range::DynamicRangeMeter;
use sonicprobe_core::frequency::Frequency;

const SAMPLE_RATE: f64 = 44_100.0;

/// Deterministic music-like signal in [-1.0, 1.0] with a slow (~0.05 Hz)
/// loudness envelope, so consecutive 3-second chunks have genuinely different
/// RMS — i.e. a non-degenerate dynamic range for the meter to measure.
fn realistic_signal(samples: usize) -> Vec<f64> {
    const PARTIALS: [(f64, f64); 5] = [
        (110.0, 0.50),
        (220.0, 0.30),
        (330.0, 0.15),
        (440.0, 0.10),
        (1760.0, 0.05),
    ];

    let mut rng = StdRng::seed_from_u64(0xD1_A2_C0FF_EE);
    let mut out = Vec::with_capacity(samples);

    for i in 0..samples {
        let t = i as f64 / SAMPLE_RATE;
        // Slow macro-dynamics across chunks (loud/quiet passages) + fast tremolo.
        let macro_env = 0.5 + 0.5 * (2.0 * PI * 0.05 * t).sin();
        let tremolo = 0.7 + 0.3 * (2.0 * PI * 3.0 * t).sin();

        let mut sample = 0.0;
        for (freq, amp) in PARTIALS {
            sample += amp * (2.0 * PI * freq * t).sin();
        }
        sample *= macro_env * tremolo;
        sample += (rng.random::<f64>() - 0.5) * 0.002;

        out.push(sample.clamp(-1.0, 1.0));
    }

    out
}

fn bench_meter(c: &mut Criterion) {
    let sample_rate = Frequency::CdQuality;
    let chunk = sample_rate.to_hz() * 3;

    // Multiples of the 3s chunk so the population is well-filled: 30 / 60 chunks.
    let sizes = [chunk * 30, chunk * 60];

    let mut group = c.benchmark_group("dynamic_range");

    for &n in &sizes {
        let signal = realistic_signal(n);

        // Visibility / cross-refactor accuracy anchor: this DR value must stay
        // stable before vs after the buffer→accumulator rewrite.
        let mut probe = DynamicRangeMeter::new(&n, &sample_rate);
        for s in &signal {
            probe.push_sample(s).expect("non-degenerate sample");
        }
        println!(
            "[dr] n={n:>9} ({} chunks)  dr={:.12} dB",
            n / chunk,
            probe.get_dr_value().get_value()
        );

        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(n), &signal, |b, sig| {
            b.iter(|| {
                let mut meter = DynamicRangeMeter::new(&n, &sample_rate);
                for sample in sig {
                    meter.push_sample(black_box(sample)).expect("sample");
                }
                black_box(meter.get_dr_value())
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_meter);
criterion_main!(benches);