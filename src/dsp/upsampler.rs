use crate::{audio_utils::catmull_rom_interpolation, dsp::Upsampler};

impl Upsampler {
    pub fn new(original_frequency: u32) -> Self {
        let multipier: u8 = {
            let ratio = (super::TARGET_FREQUENCY / original_frequency) as u8;
            if ratio < 1 { 1 } else { ratio }
        };

        Self { multipier }
    }

    #[inline]
    pub fn submit(&self, window: &[f32]) -> Vec<f32> {
        (0..self.multipier)
            .map(|k| {
                if k == 0 {
                    window[1]
                } else {
                    catmull_rom_interpolation(
                        window[0] as f64,
                        window[1] as f64,
                        window[2] as f64,
                        window[3] as f64,
                        k as f64 / self.multipier as f64,
                    )
                }
            })
            .collect()
    }
}
