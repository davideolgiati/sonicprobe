use crate::channel::builders::RMSBuilder;

impl RMSBuilder {
    pub fn new() -> RMSBuilder {
        RMSBuilder {
            partials: Vec::new(),
            count: 0,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        let mut current = (value as f64).powi(2);
        let mut index: usize = 0;

        for mut partial in self.partials.clone() {
            if current.abs() < partial.abs() {
                (current, partial) = (partial, current)
            }

            let high = current + partial;
            let low = partial - (high - current);

            if low != 0.0 {
                self.partials[index] = low;
                index += 1;
            }
            current = high
        }

        self.partials.truncate(index);
        self.partials.push(current);
        self.count += 1;
    }

    pub fn build(&self) -> f32 {
        if self.count == 0 {
            return 0.0f32;
        }

        let sum: f64 = self.partials.iter().sum();

        (sum / self.count as f64).sqrt() as f32
    }
}

#[cfg(test)]
mod tests {
    // genarati dall' AI ma rivisti da me per valuarne la correttezza
    use super::*;

    #[test]
    fn test_new_creates_empty_builder() {
        let builder = RMSBuilder::new();
        assert_eq!(builder.count, 0);
        assert_eq!(builder.partials.len(), 0);
    }

    #[test]
    fn test_add_single_value() {
        let mut builder = RMSBuilder::new();
        builder.add(3.0);
        let result = builder.build();
        assert_eq!(result, 3.0); // RMS of single value is the value itself
    }

    #[test]
    fn test_add_multiple_values() {
        let mut builder = RMSBuilder::new();
        builder.add(3.0);
        builder.add(4.0);
        builder.add(5.0);
        let result = builder.build();
        // RMS of [3, 4, 5] = sqrt((9 + 16 + 25) / 3) = sqrt(50/3) ≈ 4.08
        assert!((result - 4.08248).abs() < 0.001);
    }

    #[test]
    fn test_zero_values() {
        let mut builder = RMSBuilder::new();
        builder.add(0.0);
        builder.add(0.0);
        let result = builder.build();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_negative_values() {
        let mut builder = RMSBuilder::new();
        builder.add(-3.0);
        builder.add(-4.0);
        let result = builder.build();
        // RMS of [-3, -4] = sqrt((9 + 16) / 2) = sqrt(12.5) = 3.536
        assert!((result - 3.5355).abs() < 0.001);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let mut builder = RMSBuilder::new();
        builder.add(-3.0);
        builder.add(4.0);
        let result = builder.build();
        // RMS of [-3, 4] = sqrt((9 + 16) / 2) = sqrt(12.5) = 3.536
        assert!((result - 3.5355).abs() < 0.001);
    }

    #[test]
    fn test_build_without_adding_values() {
        let builder = RMSBuilder::new();
        let result = builder.build();
        // This tests edge case - might return 0.0 or NaN depending on implementation
        assert!(result.is_nan() || result == 0.0);
    }

    #[test]
    fn test_multiple_builds() {
        let mut builder = RMSBuilder::new();
        builder.add(3.0);
        builder.add(4.0);

        let result1 = builder.build();
        let result2 = builder.build();

        // Build should be idempotent
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_audio_signal_rms() {
        // Real case: Calculate RMS of audio samples for volume measurement
        // This simulates processing a 1024-sample audio buffer for level metering
        // in a digital audio workstation or audio effects plugin

        let mut builder = RMSBuilder::new();

        // Generate realistic audio samples: combination of fundamental + harmonics
        // Simulating a 440Hz tone (A4) with some harmonic content at 44.1kHz sample rate
        let sample_rate = 44100.0;
        let frequency = 440.0;
        let buffer_size = 1024;

        for i in 0..buffer_size {
            let t = i as f32 / sample_rate;

            // Create a complex waveform with fundamental and harmonics
            let fundamental = 0.6 * (2.0 * std::f32::consts::PI * frequency * t).sin();
            let second_harmonic = 0.3 * (2.0 * std::f32::consts::PI * frequency * 2.0 * t).sin();
            let third_harmonic = 0.1 * (2.0 * std::f32::consts::PI * frequency * 3.0 * t).sin();

            // Add some noise to make it more realistic
            let noise = 0.02 * ((i as f32 * 0.1234).sin() - 0.5);

            let sample = fundamental + second_harmonic + third_harmonic + noise;

            // Apply slight amplitude envelope to avoid clicks
            let envelope = if i < 64 {
                i as f32 / 64.0
            } else if i > buffer_size - 64 {
                (buffer_size - i) as f32 / 64.0
            } else {
                1.0
            };

            builder.add(sample * envelope);
        }

        let rms_volume = builder.build();

        // For this complex waveform, RMS should be around 0.4-0.5
        // This represents a moderately loud audio signal
        assert!(
            rms_volume > 0.35 && rms_volume < 0.55,
            "RMS volume {:.3} outside expected range for audio signal",
            rms_volume
        );

        // Test practical audio processing thresholds
        if rms_volume > 0.7 {
            // Would trigger limiter in audio processing
            panic!("Signal too hot - would cause clipping");
        } else if rms_volume > 0.5 {
            // Good level for music production
            println!("Audio level: LOUD (RMS: {:.3})", rms_volume);
        } else if rms_volume > 0.2 {
            // Normal speaking/instrument level
            println!("Audio level: MODERATE (RMS: {:.3})", rms_volume);
        } else if rms_volume > 0.05 {
            // Quiet but audible
            println!("Audio level: QUIET (RMS: {:.3})", rms_volume);
        } else {
            // Very quiet or background noise
            println!("Audio level: VERY QUIET (RMS: {:.3})", rms_volume);
        }

        // Verify the RMS is significantly different from peak amplitude
        // RMS should be lower than the peak value of 1.0 due to the waveform shape
        assert!(rms_volume < 1.0, "RMS should be less than peak amplitude");

        // For sine waves, RMS = peak / sqrt(2) ≈ peak * 0.707
        // Our complex wave should have RMS somewhere in that ballpark
        let expected_sine_rms = 0.6 * 0.707; // 0.6 was our fundamental amplitude
        assert!(
            (rms_volume - expected_sine_rms).abs() < 0.2,
            "RMS {:.3} too far from expected sine wave RMS {:.3}",
            rms_volume,
            expected_sine_rms
        );
    }
}
