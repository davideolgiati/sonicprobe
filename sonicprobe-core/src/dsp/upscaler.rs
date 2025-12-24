use crate::{
    floating_point_math::dot_product::dot_product,
    model::{HD_COEFFS, SD_COEFFS, frequency::Frequency, sonicprobe_error::SonicProbeError},
};

use std::f64;

enum FilterPhase {
    Two,
    Four,
}

pub struct Upscaler<'a> {
    phase_matrix: &'a [[f64; 12]],
    buffer: Vec<f64>,
    phases: FilterPhase,
    original_samples_index: usize,
    buffer_index: usize,
    samples: &'a [f64]
}

impl<'a> Upscaler<'a> {
    pub fn new(source: &'a[f64], source_sample_rate: Frequency) -> Result<Self, SonicProbeError> {
        let phases: FilterPhase = match source_sample_rate {
            Frequency::CdQuality | Frequency::ProAudio => FilterPhase::Four,
            Frequency::HiResDouble | Frequency::DvdAudio => FilterPhase::Two,
            Frequency::StudioMaster | Frequency::UltraHiRes => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: "upscaling for 176,400Hz and 192,000Hz not implemented".to_owned(),
                });
            }
        };

        let phase_matrix: &[[f64; 12]] = match source_sample_rate {
            Frequency::CdQuality | Frequency::ProAudio => &SD_COEFFS,
            Frequency::HiResDouble | Frequency::DvdAudio => &HD_COEFFS,
            Frequency::StudioMaster | Frequency::UltraHiRes => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: "upscaling for 176,400Hz and 192,000Hz not implemented".to_owned(),
                });
            }
        };

        let buffer = match phases {
            FilterPhase::Two => vec![0.0, 0.0],
            FilterPhase::Four => vec![0.0, 0.0, 0.0, 0.0]
        };

        let buffer_index= match phases {
            FilterPhase::Two => 2,
            FilterPhase::Four => 4
        };

        Ok(Self { 
            phase_matrix, 
            buffer,
            phases,
            original_samples_index: 0,
            buffer_index,
            samples: source
        })
    }

    #[inline]
    pub fn next_sample(&mut self) -> Option<&f64> {
        if self.buffer_index + 1 < self.buffer.len() {
            self.buffer_index = self.buffer_index + 1;
            return Some(&self.buffer[self.buffer_index])
        }

        if self.original_samples_index + 1 < self.samples.len() - 12 {
            self.update_buffer();
            self.buffer_index = 0;

            return Some(&self.buffer[0])
        }

        None
    }

    fn update_buffer(&mut self) {
        let current_index = self.original_samples_index;
        let window = &self.samples[current_index..current_index+12];
    
        match self.phases {
            FilterPhase::Two =>{
                dot_product(&self.phase_matrix[0], window, &mut self.buffer[0]);
                dot_product(&self.phase_matrix[1], window, &mut self.buffer[1]);
            },
            FilterPhase::Four => {
                dot_product(&self.phase_matrix[0], window, &mut self.buffer[0]);
                dot_product(&self.phase_matrix[1], window, &mut self.buffer[1]);
                dot_product(&self.phase_matrix[2], window, &mut self.buffer[2]);
                dot_product(&self.phase_matrix[3], window, &mut self.buffer[3]);
            }
        }

        self.original_samples_index = self.original_samples_index + 1;
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
