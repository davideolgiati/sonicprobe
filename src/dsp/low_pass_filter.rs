use crate::{constants::TARGET_SAMPLE_RATE, dsp::LowPassFilter, sonicprobe_error::SonicProbeError};

use std::{f64, sync::Arc};

impl LowPassFilter {
    pub fn new(source_sample_rate: u32) -> Result<Self, SonicProbeError> {
        let cutoff_hz: f64 = f64::from(source_sample_rate) * 0.8;
        let upscaled_sample_rate: f64 = f64::from(TARGET_SAMPLE_RATE);

        let numtaps = match super::LOW_PASS_FILTER_SIZE.try_into() {
            Ok(value) => value,
            Err(e) => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: format!("{e:?}"),
                });
            }
        };

        let coeffs: Vec<f64> = low_pass_filter(cutoff_hz, upscaled_sample_rate, numtaps)?;

        let mut coeffs_slice = [0.0f64; super::LOW_PASS_FILTER_SIZE];
        coeffs_slice.copy_from_slice(&coeffs);

        Ok(Self {
            coeffs: Arc::from(coeffs_slice),
        })
    }

    #[inline]
    pub fn submit(&self, window: &[f64]) -> f64 {
        dot_product_scalar(&self.coeffs, window)
    }
}

#[inline]
pub fn dot_product_scalar(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), 48);
    assert_eq!(b.len(), 48);

    let mut sum = [0.0f64; 4];
    let pa = a.as_ptr();
    let pb = b.as_ptr();

    for i in (7..48).step_by(8) {
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
    }

    sum[0] + sum[1] + sum[2] + sum[3]
}

fn hz_to_radian(frequency: f64, sample_rate: f64) -> f64 {
    (frequency / sample_rate) * 2.0 * f64::consts::PI
}

fn low_pass_filter(
    cutoff: f64,
    sample_rate: f64,
    numtaps: u16,
) -> Result<Vec<f64>, SonicProbeError> {
    let center_frequency: f64 = hz_to_radian(cutoff, sample_rate);
    let window_center = f64::from(numtaps - 1) / 2.0;
    let window = (0..numtaps)
        .map(|n| {
            0.46f64.mul_add(
                -((2.0 * f64::consts::PI * f64::from(n)) / f64::from(numtaps - 1)).cos(),
                0.54,
            )
        })
        .collect::<Vec<f64>>();

    // generazione
    let mut coeffs: Vec<f64> = Vec::new();
    for n in 0..numtaps {
        let offset = f64::from(n) - window_center;
        let Some(&current_value) = window.get(usize::from(n)) else {
            return Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: format!("no value for index {n}"),
            });
        };

        if offset.abs() > f64::EPSILON {
            coeffs.push(
                (center_frequency * offset).sin() / (f64::consts::PI * offset) * current_value,
            );
        } else {
            coeffs.push(center_frequency / f64::consts::PI * current_value);
        }
    }

    // normalizzazione
    let sum: f64 = coeffs.iter().sum();
    if sum != 0.0 {
        for coeff in &mut coeffs {
            *coeff /= sum;
        }
    }

    Ok(coeffs)
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
    #[allow(clippy::unwrap_used)]
    fn filter_coefficient_generation() {
        let sample_rate = 44100;
        let filter = LowPassFilter::new(sample_rate).unwrap();

        assert_eq!(filter.coeffs.len(), 48);

        let coefficient_sum: f64 = filter.coeffs.iter().sum();
        assert!(coefficient_sum > 0.0);
        assert!(coefficient_sum <= 48.0);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn filter_submit_integration() {
        let sample_rate = 48000;
        let filter = LowPassFilter::new(sample_rate).unwrap();

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
