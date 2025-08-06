use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use std::{hint::black_box, sync::Arc};

pub struct LowPassFilter {
    coeffs: *const f64,
}

fn low_pass_filter() -> Vec<f64> {
    let mut rng = rand::rng();
    let mut coeffs: Vec<f64> = vec![0.0; 48];

    // Fill the array with random values
    for i in 0..48 {
        coeffs[i] = rng.random::<f64>();
    }

    coeffs
}

impl LowPassFilter {
    pub fn new() -> Self {

        let coeffs: Vec<f64> = low_pass_filter();

        Self {
            coeffs: coeffs.as_ptr(),
        }
    }

    #[inline]
    pub fn submit(&self, window: &[f64]) -> f64 {
        dot_product_scalar(self.coeffs, window)
    }
}

#[inline]
pub fn dot_product_scalar(pa: *const f64, b: &[f64]) -> f64 {
    let mut sum = 0.0f64;
    let pb = b.as_ptr();

    for i in [7, 15, 23, 31, 39, 47] {
        unsafe {
            sum += (*pa.add(i) * *pb.add(i)) +
            (*pa.add(i - 1) * *pb.add(i - 1)) +
            (*pa.add(i - 2) * *pb.add(i - 2)) +
            (*pa.add(i - 3) * *pb.add(i - 3)) +
            (*pa.add(i - 4) * *pb.add(i - 4)) +
            (*pa.add(i - 5) * *pb.add(i - 5)) +
            (*pa.add(i - 6) * *pb.add(i - 6)) +
            (*pa.add(i - 7) * *pb.add(i - 7));
        }
    }

    sum
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();

    let mut data: Vec<f64> = vec![0.0f64; 4410000]; // 200s @ 192k

    // Fill the array with random values
    for i in 0..4410000 {
        data[i] = rng.random::<f64>();
    }

    c.bench_function("dot mul", |b|
        b.iter(|| {
            let lp = LowPassFilter::new();
            let _new_data: Vec<f64> = (0..(data.len() - 48))
            .map(|index| lp.submit( black_box(&data[index..index+48])))
            .collect();
        })
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
