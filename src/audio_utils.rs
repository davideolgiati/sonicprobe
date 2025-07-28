use std::f32;

pub fn to_dbfs(rms: f32) -> f32 {
    20.0 * rms.log10()
}

#[inline]
pub fn catmull_rom_interpolation(y0: f64, y1: f64, y2: f64, y3: f64, t: f64) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;

    0.5 * (
        2.0 * y1 +
        (-y0 + y2) * t +
        (2.0 * y0 - 5.0 * y1 + 4.0 * y2 - y3) * t2 +
        (-y0 + 3.0 * y1 - 3.0 * y2 + y3) * t3
    ) as f32
}

fn hz_to_radian(frequency: f32, sample_rate: f32) -> f32 {
    (frequency / sample_rate) * 2.0 * f32::consts::PI
}

pub fn low_pass_filter(cutoff: f32, sample_rate: f32, numtaps: i16) -> Vec<f32> {
    let center_frequency: f32 = hz_to_radian(cutoff, sample_rate);
    let window_center = (numtaps - 1) as f32 / 2.0;
    let window = (0..numtaps).map(
        |n| 0.54 - 0.46 * ((2.0 * f32::consts::PI * n as f32) / (numtaps - 1) as f32).cos()
    ).collect::<Vec<f32>>();

    // generazione
    let mut coeffs: Vec<f32> = (0..numtaps)
        .map(|n| {
            let offset = n as f32 - window_center;
            
            if offset.abs() > f32::EPSILON {
                (center_frequency * offset).sin() / (f32::consts::PI * offset) * window[n as usize]
            } else {
                center_frequency / f32::consts::PI * window[n as usize]
            }
        })
        .collect();

    // normalizzazione
    let sum: f32 = coeffs.iter().sum();
    if sum != 0.0 {
        for coeff in &mut coeffs {
            *coeff /= sum;
        }
    }

    coeffs
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for to_dbfs function
    #[test]
    fn test_to_dbfs_full_scale() {
        let result = to_dbfs(1.0);
        assert_eq!(result, 0.0); // 1.0 RMS = 0 dBFS
    }

    #[test]
    fn test_to_dbfs_half_scale() {
        let result = to_dbfs(0.5);
        assert!((result - (-6.02)).abs() < 0.1); // 0.5 ≈ -6 dBFS
    }

    #[test]
    fn test_to_dbfs_quarter_scale() {
        let result = to_dbfs(0.25);
        assert!((result - (-12.04)).abs() < 0.1); // 0.25 ≈ -12 dBFS
    }

    #[test]
    fn test_to_dbfs_very_quiet() {
        let result = to_dbfs(0.001);
        assert!((result - (-60.0)).abs() < 1.0); // 0.001 ≈ -60 dBFS
    }

    #[test]
    fn test_to_dbfs_zero() {
        let result = to_dbfs(0.0);
        assert!(result.is_infinite() && result.is_sign_negative()); // Should be -∞
    }

    #[test]
    fn test_to_dbfs_negative_infinity() {
        let result = to_dbfs(-1.0);
        assert!(result.is_nan()); // Negative input should return NaN
    }

    // Tests for catmull_rom_interpolation function
    #[test]
    fn test_catmull_rom_at_endpoints() {
        // At t=0, should return y1
        let result = catmull_rom_interpolation(0.0, 1.0, 2.0, 3.0, 0.0);
        assert!((result - 1.0).abs() < 0.001);

        // At t=1, should return y2
        let result = catmull_rom_interpolation(0.0, 1.0, 2.0, 3.0, 1.0);
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_catmull_rom_midpoint() {
        // At t=0.5, should be smooth interpolation between y1 and y2
        let result = catmull_rom_interpolation(0.0, 1.0, 3.0, 4.0, 0.5);
        assert!(result > 1.0 && result < 3.0); // Should be between y1 and y2
        assert!((result - 2.0).abs() < 0.5); // Should be close to midpoint
    }

    #[test]
    fn test_catmull_rom_linear_data() {
        // For linear data, should produce linear interpolation
        let result = catmull_rom_interpolation(1.0, 2.0, 3.0, 4.0, 0.5);
        assert!((result - 2.5).abs() < 0.001); // Exact midpoint for linear data
    }

    #[test]
    fn test_catmull_rom_constant_data() {
        // All same values should return that value
        let result = catmull_rom_interpolation(5.0, 5.0, 5.0, 5.0, 0.3);
        assert!((result - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_catmull_rom_smooth_curve() {
        // Test smooth curve behavior
        let y0 = 0.0;
        let y1 = 1.0;
        let y2 = 0.0;
        let y3 = -1.0;
        
        let result_quarter = catmull_rom_interpolation(y0, y1, y2, y3, 0.25);
        let result_half = catmull_rom_interpolation(y0, y1, y2, y3, 0.5);
        let result_three_quarter = catmull_rom_interpolation(y0, y1, y2, y3, 0.75);
        
        // Should create smooth transition from y1 to y2
        assert!(result_quarter > result_half); // Decreasing
        assert!(result_half > result_three_quarter); // Still decreasing
    }

    // Tests for hz_to_radian function
    #[test]
    fn test_hz_to_radian_nyquist() {
        let sample_rate = 44100.0;
        let nyquist = sample_rate / 2.0;
        let result = hz_to_radian(nyquist, sample_rate);
        assert!((result - std::f32::consts::PI).abs() < 0.001); // Nyquist = π radians
    }

    #[test]
    fn test_hz_to_radian_dc() {
        let result = hz_to_radian(0.0, 44100.0);
        assert_eq!(result, 0.0); // DC = 0 radians
    }

    #[test]
    fn test_hz_to_radian_1khz() {
        let result = hz_to_radian(1000.0, 44100.0);
        let expected = 2.0 * std::f32::consts::PI * 1000.0 / 44100.0;
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_hz_to_radian_quarter_nyquist() {
        let sample_rate = 48000.0;
        let quarter_nyquist = sample_rate / 8.0; // 6kHz
        let result = hz_to_radian(quarter_nyquist, sample_rate);
        let expected = std::f32::consts::PI / 4.0;
        assert!((result - expected).abs() < 0.001);
    }

    // Tests for low_pass_filter function
    #[test]
    fn test_low_pass_filter_basic_properties() {
        let coeffs = low_pass_filter(1000.0, 44100.0, 64);
        
        // Should return correct number of taps
        assert_eq!(coeffs.len(), 64);
        
        // FIR coefficients should sum to approximately 1.0 for unity gain at DC
        let sum: f32 = coeffs.iter().sum();
        assert!((sum - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_low_pass_filter_symmetry() {
        let coeffs = low_pass_filter(2000.0, 48000.0, 32);

        // FIR low-pass filter should be symmetric
        let len = coeffs.len();
        for i in 0..len/2 {
            assert!((coeffs[i] - coeffs[len - 1 - i]).abs() < 0.001);
        }
    }

    #[test]
    fn test_low_pass_filter_different_lengths() {
        let coeffs_short = low_pass_filter(1000.0, 44100.0, 16);
        let coeffs_long = low_pass_filter(1000.0, 44100.0, 64);
        
        assert_eq!(coeffs_short.len(), 16);
        assert_eq!(coeffs_long.len(), 64);
        
        // Longer filter should have more precise frequency response
        // Both should sum to approximately 1.0
        let sum_short: f32 = coeffs_short.iter().sum();
        let sum_long: f32 = coeffs_long.iter().sum();
        assert!((sum_short - 1.0).abs() < 0.1);
        assert!((sum_long - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_low_pass_filter_extreme_cutoffs() {
        // Very low cutoff
        let coeffs_low = low_pass_filter(100.0, 44100.0, 32);
        assert_eq!(coeffs_low.len(), 32);
        
        // High cutoff (near Nyquist)
        let coeffs_high = low_pass_filter(20000.0, 44100.0, 32);
        assert_eq!(coeffs_high.len(), 32);
        
        // Both should be valid filters
        let sum_low: f32 = coeffs_low.iter().sum();
        let sum_high: f32 = coeffs_high.iter().sum();
        assert!((sum_low - 1.0).abs() < 0.2);
        assert!((sum_high - 1.0).abs() < 0.2);
    }

    #[test]
    fn test_low_pass_filter_odd_even_taps() {
        let coeffs_even = low_pass_filter(1000.0, 44100.0, 32);
        let coeffs_odd = low_pass_filter(1000.0, 44100.0, 33);
        
        assert_eq!(coeffs_even.len(), 32);
        assert_eq!(coeffs_odd.len(), 33);
        
        // Both should be valid symmetric filters
        // Check symmetry for even length
        for i in 0..16 {
            assert!((coeffs_even[i] - coeffs_even[31 - i]).abs() < 0.001);
        }
        
        // Check symmetry for odd length
        for i in 0..16 {
            assert!((coeffs_odd[i] - coeffs_odd[32 - i]).abs() < 0.001);
        }
    }

    #[test]
    fn test_low_pass_filter_center_tap() {
        let coeffs = low_pass_filter(1000.0, 44100.0, 65); // Odd length
        
        // For low-pass filter, center tap should be the largest coefficient
        let center_idx = coeffs.len() / 2;
        let center_value = coeffs[center_idx];
        
        // All other coefficients should be smaller or equal
        for (i, &coeff) in coeffs.iter().enumerate() {
            if i != center_idx {
                assert!(coeff <= center_value, 
                       "Coefficient at index {} ({}) larger than center ({})", 
                       i, coeff, center_value);
            }
        }
    }

    #[test]
    fn test_low_pass_filter_frequency_response_basic() {
        let coeffs = low_pass_filter(5000.0, 44100.0, 64);
        
        // Simple frequency response test at DC (should be close to 1.0)
        let dc_response: f32 = coeffs.iter().sum();
        assert!((dc_response - 1.0).abs() < 0.1);
        
        // At very high frequencies, response should be much smaller
        // This is a simplified test - real frequency response testing would use DFT
        let mut high_freq_response = 0.0f32;
        for (i, &coeff) in coeffs.iter().enumerate() {
            // Simulate high frequency (alternating signs)
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            high_freq_response += coeff * sign;
        }
        
        // High frequency response should be much smaller than DC
        assert!(high_freq_response.abs() < dc_response * 0.5);
    }
}