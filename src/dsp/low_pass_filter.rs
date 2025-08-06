use crate::{
    audio_utils::low_pass_filter,
    constants::UPSAMPLE_TARGET_FREQUENCY,
    dsp::LowPassFilter,
};

use std::{sync::Arc};

impl LowPassFilter {
    pub fn new(original_frequency: u32) -> Self {
        let cutoff_hz: f64 = f64::from(original_frequency) * 0.8;
        let upsampled_freq: f64 = f64::from(UPSAMPLE_TARGET_FREQUENCY);

        let numtaps = match super::LOW_PASS_FILTER_SIZE.try_into() {
            Ok(value) => value,
            Err(e) => panic!("{e:?}"),
        };

        let coeffs: Vec<f64> = low_pass_filter(cutoff_hz, upsampled_freq, numtaps);

        let mut coeffs_slice = [0.0f64; super::LOW_PASS_FILTER_SIZE];
        coeffs_slice.copy_from_slice(&coeffs);

        Self {
            coeffs: Arc::from(coeffs_slice),
        }
    }

    #[inline]
    pub fn submit(&self, window: &[f64]) -> f64 {
        dot_product_scalar(&self.coeffs, window)
    }
}

#[inline]
pub fn dot_product_scalar(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = [0.0f64; 4];
    let pa = a.as_ptr();
    let pb = b.as_ptr();
    let mut i = 7;

    while i <= 48 {
        unsafe {
            sum[0] += *pa.add(i) * *pb.add(i);
            sum[1] += *pa.add(i - 1) * *pb.add(i - 1);
            sum[2] += *pa.add(i - 2) * *pb.add(i - 2);
            sum[3] += *pa.add(i - 3) * *pb.add(i - 3);
            sum[0] += *pa.add(i - 4) * *pb.add(i - 4);
            sum[1] += *pa.add(i - 5) * *pb.add(i - 5);
            sum[2] += *pa.add(i - 6) * *pb.add(i - 6);
            sum[3] += *pa.add(i - 7) * *pb.add(i - 7);
        }
        i += 8;
    }

    sum[0] + sum[1] + sum[2] + sum[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product_bounds_safety() {
        let coefficients = [1.0; 48];
        let window_data = [2.0; 48];
        
        let result = dot_product_scalar(&coefficients, &window_data);
        let expected = 48.0 * 2.0;
        
        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test] 
    fn filter_coefficient_generation() {
        let sample_rate = 44100;
        let filter = LowPassFilter::new(sample_rate);
        
        assert_eq!(filter.coeffs.len(), 48);
        
        let coefficient_sum: f64 = filter.coeffs.iter().sum();
        assert!(coefficient_sum > 0.0);
        assert!(coefficient_sum <= 48.0);
    }

    #[test]
    fn filter_submit_integration() {
        let sample_rate = 48000;
        let filter = LowPassFilter::new(sample_rate);
        
        let dc_signal = [1.0; 48];
        let dc_response = filter.submit(&dc_signal);
        
        let alternating_signal: Vec<f64> = (0..48)
            .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
            .collect();
        let alternating_response = filter.submit(&alternating_signal);
        
        assert!(dc_response > alternating_response);
        assert!(dc_response > 0.0);
    }
}