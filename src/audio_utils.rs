use std::f64;

use crate::audio_file::Signal;

pub fn to_dbfs(dc: f64) -> f64 {
    20.0 * dc.log10()
}

pub fn catmull_rom_interpolation(window: &Signal, start: usize, t: f64) -> Result<f64, String> {
    let b0 = 0.5f64.mul_add(-t, t.mul_add(t, -(0.5 * t.powi(3))));
    let b1 = 1.5f64.mul_add(t.powi(3), -(2.5 * t.powi(2))) + 1.0;
    let b2 = 0.5f64.mul_add(t, (-1.5f64).mul_add(t.powi(3), 2.0 * t.powi(2)));
    let b3 = 0.5f64.mul_add(t.powi(3), -(0.5 * t.powi(2)));

    assert!((b0 + b1 + b2 + b3) - 1.0 < f64::EPSILON);

    let y_minus1 = match window.get(start) {
        Some(&value) => value,
        None => return Err(format!("catmull rom: index {start} out of boundaries"))
    };

    let y0 = match window.get(start + 1) {
        Some(&value) => value,
        None => return Err(format!("catmull rom: index {start} out of boundaries"))
    };

    let y1 = match window.get(start + 2) {
        Some(&value) => value,
        None => return Err(format!("catmull rom: index {start} out of boundaries"))
    };

    let y2 = match window.get(start + 3) {
        Some(&value) => value,
        None => return Err(format!("catmull rom: index {start} out of boundaries"))
    };

    Ok(y2.mul_add(b3, y1.mul_add(b2, y_minus1.mul_add(b0, y0 * b1))))
}

fn hz_to_radian(frequency: f64, sample_rate: f64) -> f64 {
    (frequency / sample_rate) * 2.0 * f64::consts::PI
}

pub fn low_pass_filter(cutoff: f64, sample_rate: f64, numtaps: u16) -> Vec<f64> {
    let center_frequency: f64 = hz_to_radian(cutoff, sample_rate);
    let window_center = f64::from(numtaps - 1) / 2.0;
    let window = (0..numtaps)
        .map(|n| 0.46f64.mul_add(-((2.0 * f64::consts::PI * f64::from(n)) / f64::from(numtaps - 1)).cos(), 0.54))
        .collect::<Vec<f64>>();

    // generazione
    let mut coeffs: Vec<f64> = (0..numtaps)
        .map(|n| {
            let offset = f64::from(n) - window_center;
            let current_window_value = match window.get(usize::from(n)) {
                Some(&value) => value,
                None => panic!("low pass filter: no value for index {n}")
            };

            if offset.abs() > f64::EPSILON {
                (center_frequency * offset).sin() / (f64::consts::PI * offset) * current_window_value
            } else {
                center_frequency / f64::consts::PI * current_window_value
            }
        })
        .collect();

    // normalizzazione
    let sum: f64 = coeffs.iter().sum();
    if sum != 0.0 {
        for coeff in &mut coeffs {
            *coeff /= sum;
        }
    }

    coeffs
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_to_dbfs_conversion() {
        // Test 1: Full scale (1.0 RMS) should be 0 dBFS.
        assert_eq!(to_dbfs(1.0), 0.0, "Full scale did not result in 0 dBFS");

        // Test 2: A common value (0.5 RMS) should be approximately -6.02 dBFS.
        assert!(
            (to_dbfs(0.5) - (-6.02)).abs() < 0.01,
            "Half scale was not approx -6 dBFS"
        );

        // Test 3: Zero or silence (0.0 RMS) should result in negative infinity.
        let zero_dbfs = to_dbfs(0.0);
        assert!(
            zero_dbfs.is_infinite() && zero_dbfs.is_sign_negative(),
            "0.0 RMS should be -inf dBFS"
        );

        // Test 4: Invalid input (negative RMS) should result in NaN (Not a Number).
        assert!(to_dbfs(-1.0).is_nan(), "Negative input did not produce NaN");

        // Test 5: Over-unity gain should produce a positive dBFS value.
        // This simulates a clipped signal and verifies the math holds for values > 1.0.
        assert!(
            (to_dbfs(2.0) - 6.02).abs() < 0.01,
            "Over-unity gain signal was not handled correctly"
        );
    }

    #[test]
    fn test_catmull_rom_interpolation_properties() {
        let arr = [0.0f32, 1.0f32, 2.0f32, 3.0f32];
        let data: Signal = Arc::new(arr);
        // Test 1: At t=0, the output should exactly match the second point (y1).
        assert!((catmull_rom_interpolation(&data, 0, 0.0) - 1.0).abs() < 1e-6);

        // Test 2: At t=1, the output should exactly match the third point (y2).
        assert!((catmull_rom_interpolation(&data, 0, 1.0) - 2.0).abs() < 1e-6);

        // Test 3: With linearly spaced points, the interpolation should also be linear.
        let midpoint = catmull_rom_interpolation(&data, 0, 0.5);
        assert!(
            (midpoint - 2.5).abs() < 1e-6,
            "Interpolation of linear data was not linear"
        );
    }

    #[test]
    fn test_hz_to_radian_conversions() {
        let sample_rate = 44100.0;

        // Test 1: 0 Hz (DC) should correspond to 0 radians.
        assert_eq!(
            hz_to_radian(0.0, sample_rate),
            0.0,
            "DC frequency was not 0 radians"
        );

        // Test 2: The Nyquist frequency (sample_rate / 2) should correspond to PI radians.
        let nyquist_freq = sample_rate / 2.0;
        assert!(
            (hz_to_radian(nyquist_freq, sample_rate) - PI).abs() < 1e-6,
            "Nyquist frequency was not PI radians"
        );
    }

    #[test]
    fn test_low_pass_filter_structural_properties() {
        // Test with an odd number of taps to easily check the center tap.
        let coeffs_odd = low_pass_filter(5000.0, 44100.0, 65);
        assert_eq!(
            coeffs_odd.len(),
            65,
            "Filter did not produce the correct number of taps (odd)"
        );

        // Test 1: The sum of coefficients (DC gain) must be approximately 1.0.
        let sum_odd: f32 = coeffs_odd.iter().sum();
        assert!(
            (sum_odd - 1.0).abs() < 1e-6,
            "Coefficients (odd) do not sum to 1.0"
        );

        // Test 2: The filter must be symmetrical for a linear phase response.
        let len_odd = coeffs_odd.len();
        for i in 0..len_odd / 2 {
            assert!(
                (coeffs_odd[i] - coeffs_odd[len_odd - 1 - i]).abs() < 1e-6,
                "Filter (odd) is not symmetrical"
            );
        }

        // Test 3: For a low-pass filter, the center tap must be the largest coefficient.
        let center_idx = len_odd / 2;
        let center_val = coeffs_odd[center_idx];
        assert!(
            coeffs_odd.iter().all(|&c| c <= center_val),
            "Center tap was not the max coefficient"
        );
    }

    #[test]
    fn test_low_pass_filter_frequency_response() {
        let sample_rate = 44100.0;
        let cutoff_hz = 5000.0;
        let num_taps = 101;
        let coeffs = low_pass_filter(cutoff_hz, sample_rate, num_taps);

        // Helper function to calculate the magnitude response at a given frequency
        // This is a basic Discrete Fourier Transform (DFT) for a specific frequency 'w'.
        let get_magnitude_at_radian = |w: f32, taps: &Vec<f32>| -> f32 {
            let mut real = 0.0;
            let mut imag = 0.0;
            for (n, &coeff) in taps.iter().enumerate() {
                real += coeff * (w * n as f32).cos();
                imag -= coeff * (w * n as f32).sin();
            }
            (real.powi(2) + imag.powi(2)).sqrt()
        };

        // Test 1: Gain at DC (0 Hz) should be 1.0 (0 dB).
        let dc_radian = hz_to_radian(0.0, sample_rate);
        let dc_gain = get_magnitude_at_radian(dc_radian, &coeffs);
        assert!((dc_gain - 1.0).abs() < 1e-6, "Gain at DC is not 1.0");

        // Test 2: Gain at the cutoff frequency should be approx. 0.5 (-6 dB).
        // This is a key indicator that the cutoff is being implemented correctly.
        let cutoff_radian = hz_to_radian(cutoff_hz, sample_rate);
        let cutoff_gain = get_magnitude_at_radian(cutoff_radian, &coeffs);
        assert!(
            (cutoff_gain - 0.5).abs() < 0.05,
            "Gain at cutoff frequency is not approx. 0.5 (-6 dB)"
        );

        // Test 3: Gain at Nyquist frequency should be very low (high attenuation).
        let nyquist_radian = PI;
        let nyquist_gain = get_magnitude_at_radian(nyquist_radian, &coeffs);
        let nyquist_gain_db = 20.0 * nyquist_gain.log10();
        assert!(
            nyquist_gain_db < -40.0,
            "Filter has poor stopband attenuation at Nyquist"
        );
    }
}
*/
