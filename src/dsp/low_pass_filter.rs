use crate::{
    floating_point_math::dot_product::dot_product,
    model::{frequency::Frequency, sonicprobe_error::SonicProbeError, HD_COEFFS, SD_COEFFS},
};

use std::f64;

pub struct LowPassFilter<'a> {
    coeffs: &'a [[f64; 12]],
}

impl LowPassFilter<'_> {
    #[allow(clippy::unreadable_literal)]
    pub fn new(source_sample_rate: Frequency) -> Result<Self, SonicProbeError> {
        let coeffs: &[[f64; 12]] = match source_sample_rate {
            Frequency::CdQuality | Frequency::ProAudio => &SD_COEFFS,
            Frequency::HiResDouble | Frequency::DvdAudio => &HD_COEFFS,
            Frequency::StudioMaster | Frequency::UltraHiRes => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: "upscaling for 176,400Hz and 192,000Hz not implemented".to_owned(),
                });
            }
        };

        Ok(Self {
            coeffs,
        })
    }

    #[inline]
    pub fn submit(&self, window: &[f64]) -> impl Iterator<Item = f64> {
        (0..self.coeffs.len()).map(|index| dot_product(&self.coeffs[index], window))
    }
}

/*
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
*/
