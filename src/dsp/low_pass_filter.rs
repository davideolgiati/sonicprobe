use crate::{
    audio_file::Signal, audio_utils::low_pass_filter, constants::{LOW_PASS_FILTER_SIZE, UPSAMPLE_TARGET_FREQUENCY}, dsp::LowPassFilter
};

use std::{arch::is_x86_feature_detected, process};

impl LowPassFilter {
    pub fn new(original_frequency: u32) -> Self {
        let cutoff_hz: f64 = original_frequency as f64 * 0.8;
        let upsampled_freq: f64 = UPSAMPLE_TARGET_FREQUENCY as f64;

        let numtaps = match super::LOW_PASS_FILTER_SIZE.try_into() {
            Ok(value) => value,
            Err(e) => panic!("{e:?}")
        };

        let coeffs: Vec<f64> =
            low_pass_filter(cutoff_hz, upsampled_freq, numtaps);

        let mut coeffs_slice = [0.0f64; super::LOW_PASS_FILTER_SIZE];
        coeffs_slice.copy_from_slice(&coeffs);

        Self {
            coeffs: coeffs_slice,
        }
    }

    #[inline]
    pub fn submit(&self, window: &Signal, start: usize, end: usize) -> f64 {
        let current_window = match window.get(start..end) {
            Some(array) => array,
            None => {
                println!("error: low pass filter can't slice signal form sample {start} to {end}");
                process::exit(1);
            }
        };

        let window_slice: &[f64; super::LOW_PASS_FILTER_SIZE] =
            current_window.try_into().unwrap_or_else(|_| {
                panic!(
                    "Window must be exactly {} elements, got {}",
                    LOW_PASS_FILTER_SIZE,
                    window.len()
                )
            });

        dot_product(&self.coeffs, window_slice)
    }
}

fn dot_product(coeffs: &[f64; LOW_PASS_FILTER_SIZE], samples: &[f64; LOW_PASS_FILTER_SIZE]) -> f64 {
    if is_x86_feature_detected!("avx2") {
        unsafe { dot_product_avx2(coeffs, samples) }
    } else {
        dot_product_scalar::<{ LOW_PASS_FILTER_SIZE }>(coeffs, samples)
    }
}

#[inline(always)]
pub fn dot_product_scalar<const N: usize>(a: &[f64;N], b: &[f64;N]) -> f64 {
    assert_eq!(a.len(), b.len(), "Slices must have the same length");
    
    let mut sum0 = 0.0f64;
    let mut sum1 = 0.0f64;
    let mut sum2 = 0.0f64;
    let mut sum3 = 0.0f64;
    let pa = a.as_ptr();
    let pb = b.as_ptr();
    let mut i = 0;
    
    while i + 16 <= N {
        unsafe {
            sum0 += *pa.add(i) * *pb.add(i);
            sum1 += *pa.add(i + 1) * *pb.add(i + 1);
            sum2 += *pa.add(i + 2) * *pb.add(i + 2);
            sum3 += *pa.add(i + 3) * *pb.add(i + 3);
            sum0 += *pa.add(i + 4) * *pb.add(i + 4);
            sum1 += *pa.add(i + 5) * *pb.add(i + 5);
            sum2 += *pa.add(i + 6) * *pb.add(i + 6);
            sum3 += *pa.add(i + 7) * *pb.add(i + 7);
            sum0 += *pa.add(i + 8) * *pb.add(i + 8);
            sum1 += *pa.add(i + 9) * *pb.add(i + 9);
            sum2 += *pa.add(i + 10) * *pb.add(i + 10);
            sum3 += *pa.add(i + 11) * *pb.add(i + 11);
            sum0 += *pa.add(i + 12) * *pb.add(i + 12);
            sum1 += *pa.add(i + 13) * *pb.add(i + 13);
            sum2 += *pa.add(i + 14) * *pb.add(i + 14);
            sum3 += *pa.add(i + 15) * *pb.add(i + 15);
        }
        i += 16;
    }
    
    // Handle any leftover elements
    while i < N {
        unsafe {
            sum0 += *pa.add(i) * *pb.add(i);
        }
        i += 1;
    }
    
    sum0 + sum1 + sum2 + sum3
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn dot_product_avx2(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "Slices must have the same length");
    
    unsafe {
        use std::arch::x86_64::*;
        const CHUNK: usize = 4; // AVX2 processes 4 f64 values at once
        let mut sum = _mm256_setzero_pd();
        let pa = a.as_ptr();
        let pb = b.as_ptr();
        let len = a.len();
        let mut i = 0;
        
        while i + CHUNK <= len {
            let va = _mm256_loadu_pd(pa.add(i));
            let vb = _mm256_loadu_pd(pb.add(i));
            sum = _mm256_add_pd(sum, _mm256_mul_pd(va, vb));
            i += CHUNK;
        }
        
        // Extract and sum the 4 f64 values from the AVX2 register
        let mut tmp = [0f64; CHUNK];
        _mm256_storeu_pd(tmp.as_mut_ptr(), sum);
        let mut total = tmp.iter().sum::<f64>();
        
        // Handle any remaining elements
        while i < len {
            total += *pa.add(i) * *pb.add(i);
            i += 1;
        }
        
        total
    }
}