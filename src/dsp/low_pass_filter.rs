use crate::{
    audio_utils::low_pass_filter,
    dsp::{LOW_PASS_FILTER_SIZE, LowPassFilter},
};

use std::{arch::is_x86_feature_detected, sync::Arc};

impl LowPassFilter {
    pub fn new(original_frequency: u32) -> Self {
        let cutoff_hz: f32 = (original_frequency as f32) * 0.8;
        let upsampled_freq: f32 = super::TARGET_FREQUENCY as f32;

        let mut coeffs: Vec<f32> =
            low_pass_filter(cutoff_hz, upsampled_freq, super::LOW_PASS_FILTER_SIZE);

        coeffs.reverse();

        let mut coeffs_slice = [0.0f32; super::LOW_PASS_FILTER_SIZE];
        coeffs_slice.copy_from_slice(&coeffs);

        Self {
            coeffs: coeffs_slice,
        }
    }

    #[inline]
    pub fn submit(&self, window: &Arc<[f32]>, start: usize, end: usize) -> f32 {
        let window_slice: &[f32; super::LOW_PASS_FILTER_SIZE] = window[start..end]
            .try_into().unwrap_or_else(|_| {
                panic!(
                    "Window must be exactly {} elements, got {}",
                    LOW_PASS_FILTER_SIZE,
                    window.len()
                )
            });
        dot_product(&self.coeffs, window_slice)
    }
}

fn dot_product(
    coeffs: &[f32; super::LOW_PASS_FILTER_SIZE],
    samples: &[f32; super::LOW_PASS_FILTER_SIZE],
) -> f32 {
    if is_x86_feature_detected!("avx2") {
        unsafe { dot_product_avx2(coeffs, samples) }
    } else {
        dot_product_scalar::<{ super::LOW_PASS_FILTER_SIZE }>(coeffs, samples)
    }
}

#[inline(always)]
pub fn dot_product_scalar<const N: usize>(a: &[f32; N], b: &[f32; N]) -> f32 {
    let mut sum0 = 0.0f32;
    let mut sum1 = 0.0f32;
    let mut sum2 = 0.0f32;
    let mut sum3 = 0.0f32;

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
unsafe fn dot_product_avx2(
    a: &[f32; super::LOW_PASS_FILTER_SIZE],
    b: &[f32; super::LOW_PASS_FILTER_SIZE],
) -> f32 {
    unsafe {
        use std::arch::x86_64::*;
        const CHUNK: usize = 8;

        let mut sum = _mm256_setzero_ps();
        let pa = a.as_ptr();
        let pb = b.as_ptr();
        let mut i = 0;

        while i + CHUNK <= super::LOW_PASS_FILTER_SIZE {
            let va = _mm256_loadu_ps(pa.add(i));
            let vb = _mm256_loadu_ps(pb.add(i));
            sum = _mm256_add_ps(sum, _mm256_mul_ps(va, vb));
            i += CHUNK;
        }

        let mut tmp = [0f32; CHUNK];
        _mm256_storeu_ps(tmp.as_mut_ptr(), sum);
        let mut total = tmp.iter().sum::<f32>();

        while i < super::LOW_PASS_FILTER_SIZE {
            total += *pa.add(i) * *pb.add(i);
            i += 1;
        }
        total
    }
}
