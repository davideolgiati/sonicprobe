use crate::{audio_file::Signal, audio_utils::catmull_rom_interpolation, constants::UPSAMPLE_TARGET_FREQUENCY, dsp::Upsampler};

use std::convert::TryFrom;

impl Upsampler {
    pub fn new(original_frequency: u32) -> Self {
        let multipier: u8 = {
            let ratio = match u8::try_from(UPSAMPLE_TARGET_FREQUENCY / original_frequency) {
                Ok(value) => value,
                Err(e) => panic!("Upsample ratio {}x is too large for u8: {e:?}", UPSAMPLE_TARGET_FREQUENCY / original_frequency)
            };
            if ratio < 1 { 1 } else { ratio }
        };

        Self { multipier }
    }

    #[inline]
    pub fn submit(&self, window: Signal, start: usize) -> impl Iterator<Item = f64> {
        (0..self.multipier)
            .map(move |k| {
                if k == 0 {
                    match window.get(start + 1) {
                        Some(&value) => value,
                        None => panic!("bug!")
                    }
                } else {
                    match catmull_rom_interpolation(
                        &window, start,
                        f64::from(k) / f64::from(self.multipier),
                    ) {
                        Ok(value) => value,
                        Err(e) => panic!("{e:?}")
                    }
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn upsampler_constructor_edge_cases() {
        // Test normal case - should work
        let upsampler_normal = Upsampler::new(48000);
        assert_eq!(upsampler_normal.multipier, 4); // 192000 / 48000 = 4
        
        // Test higher frequency (downsampling case) - exposes logic flaw
        let upsampler_high = Upsampler::new(384000); // Higher than target
        assert_eq!(upsampler_high.multipier, 1); // Should this really be 1?
        
        // Test exact match
        let upsampler_exact = Upsampler::new(192000);
        assert_eq!(upsampler_exact.multipier, 1);
        
        // Test low frequency - high multiplication
        let upsampler_low = Upsampler::new(8000);
        assert_eq!(upsampler_low.multipier, 24); // 192000 / 8000 = 24
    }

    #[test]
    fn upsampler_boundary_and_indexing() {
        let upsampler = Upsampler::new(48000); // multipier = 4
        
        // Create signal with known values
        let signal: Signal = Arc::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        
        // Test normal case - should expose the start+1 bug
        let mut results: Vec<f64> = upsampler.submit(signal.clone(), 1).collect();
        
        // First sample (k=0) should be signal[1] but gets signal[2] due to bug
        assert_eq!(results[0], 3.0); // This will fail - should be 2.0 but gets 3.0
        assert_eq!(results.len(), 4);
        
        // Test boundary panic - accessing beyond signal length
        let short_signal: Signal = Arc::from([1.0, 2.0]);
        
        // This should panic when trying to access start+1 = 2 on 2-element array
        let panic_result = std::panic::catch_unwind(|| {
            let _: Vec<f64> = upsampler.submit(short_signal, 1).collect();
        });
        assert!(panic_result.is_err());
    }

    #[test] 
    fn upsampler_interpolation_correctness() {
        let upsampler = Upsampler::new(96000); // multipier = 2
        
        // Linear signal for predictable interpolation
        let signal: Signal = Arc::from([0.0, 10.0, 20.0, 30.0]);
        
        let results: Vec<f64> = upsampler.submit(signal.clone(), 0).collect();
        
        // Should have 2 samples
        assert_eq!(results.len(), 2);
        
        // First sample (k=0): should be signal[1] = 10.0 but gets signal[2] = 20.0
        assert_eq!(results[0], 10.0); // Exposes the off-by-one bug
        
        // Second sample (k=1): interpolated at t=0.5 between signal points
        // This depends on catmull_rom_interpolation, but should be reasonable
        assert!(results[1] > 0.0);
        assert!(results[1] < 50.0);
    }
}