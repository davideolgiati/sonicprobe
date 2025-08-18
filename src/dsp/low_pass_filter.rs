use crate::{
    audio_file::types::Frequency, constants::TARGET_SAMPLE_RATE, dsp::{dot_product::dot_product, LowPassFilter},
    sonicprobe_error::SonicProbeError,
};

use std::{f64, sync::Arc};

impl LowPassFilter {
    pub fn new(source_sample_rate: Frequency) -> Result<Self, SonicProbeError> {
        let numtaps = match source_sample_rate {
            Frequency::CdQuality | Frequency::ProAudio => 48,
            Frequency::HiResDouble | Frequency::DvdAudio => 24,
            Frequency::StudioMaster | Frequency::UltraHiRes => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: "upscaling for 176,400Hz and 192,000Hz not implemented".to_owned(),
                });
            }
        };

        let coeffs: Vec<f64> = low_pass_filter(source_sample_rate, numtaps)?;

        let phases_count = match source_sample_rate {
            Frequency::CdQuality | Frequency::ProAudio => 4,
            Frequency::HiResDouble | Frequency::DvdAudio => 2,
            Frequency::StudioMaster | Frequency::UltraHiRes => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: "upscaling for 176,400Hz and 192,000Hz not implemented".to_owned(),
                });
            }
        };

        let polyfir_coeffs: Vec<[f64; 12]> = {
            let mut phases = vec![[0.0; 12]; phases_count];
            for (tap, coeff) in coeffs.chunks(phases_count).enumerate() {
                for phase in 0..phases_count {
                    phases[phase][tap] = coeff[phase];
                }
            }
            phases
        };

        Ok(Self {
            coeffs: Arc::from(polyfir_coeffs),
        })
    }

    #[inline]
    pub fn submit(&self, window: &[f64]) -> impl Iterator<Item = f64> {
        self.coeffs
            .iter()
            .map(|coeffs| dot_product(coeffs, window))
    }
}



fn hz_to_radian(cutoff: f64) -> f64 {
    (cutoff / TARGET_SAMPLE_RATE) * 2.0 * f64::consts::PI
}

fn low_pass_filter(sample_rate: Frequency, numtaps: u16) -> Result<Vec<f64>, SonicProbeError> {
    let cutoff: f64 = match sample_rate {
        Frequency::CdQuality => 22050.0,
        Frequency::ProAudio => 24000.0,
        Frequency::HiResDouble => 44100.0,
        Frequency::DvdAudio => 48000.0,
        Frequency::UltraHiRes => 88200.0,
        Frequency::StudioMaster => 96000.0,
    };

    let signal_boost: f64 = match sample_rate {
        Frequency::ProAudio | Frequency::CdQuality => 4.0,
        Frequency::HiResDouble | Frequency::DvdAudio => 2.0,
        Frequency::UltraHiRes | Frequency::StudioMaster => 1.0,
    };

    let center_frequency: f64 = hz_to_radian(cutoff);
    let window_center = f64::from(numtaps - 1) / 2.0;
    let window = (0..numtaps)
        .map(|n| {
            // Hann window
            0.5f64.mul_add(
                -((2.0 * f64::consts::PI * f64::from(n)) / f64::from(numtaps - 1)).cos(),
                0.5,
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
            *coeff *= signal_boost;
        }
    }

    Ok(coeffs)
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
