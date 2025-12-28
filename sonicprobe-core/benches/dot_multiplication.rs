use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use std::hint::black_box;
pub struct LowPassFilter {
    coeffs: Vec<f64>,
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

impl Default for LowPassFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl LowPassFilter {
    
    pub fn new() -> Self {

        let coeffs: Vec<f64> = low_pass_filter();

        Self {
            coeffs,
        }
    }

    #[inline]
    pub fn submit(&self, window: &[f64]) -> f64 {
        dot_product_scalar(&self.coeffs, window)
    }
}

#[inline]
pub fn dot_product_scalar(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), 12);
    assert_eq!(right.len(), 12);

    let mut sum = [0.0f64; 4];
    let letf_ptr = left.as_ptr();
    let right_ptr = right.as_ptr();

    unsafe {
        sum[0] += *letf_ptr * *right_ptr;
        sum[1] += *letf_ptr.add(1) * *right_ptr.add(1);
        sum[2] += *letf_ptr.add(2) * *right_ptr.add(2);
        sum[3] += *letf_ptr.add(3) * *right_ptr.add(3);
        sum[0] += *letf_ptr.add(4) * *right_ptr.add(4);
        sum[1] += *letf_ptr.add(5) * *right_ptr.add(5);
        sum[2] += *letf_ptr.add(6) * *right_ptr.add(6);
        sum[3] += *letf_ptr.add(7) * *right_ptr.add(7);
        sum[0] += *letf_ptr.add(8) * *right_ptr.add(8);
        sum[1] += *letf_ptr.add(9) * *right_ptr.add(9);
        sum[2] += *letf_ptr.add(10) * *right_ptr.add(10);
        sum[3] += *letf_ptr.add(11) * *right_ptr.add(11);
    }

    sum[0] + sum[1] + sum[2] + sum[3]
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();

    let mut data: Vec<f64> = vec![0.0f64; 4_410_000]; // 200s @ 192k

    // Fill the array with random values
    for i in 0..4_410_000 {
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
