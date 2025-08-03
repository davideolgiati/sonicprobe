use std::sync::Arc;

use crate::{audio_utils::catmull_rom_interpolation, constants::UPSAMPLE_TARGET_FREQUENCY, dsp::Upsampler};

impl Upsampler {
    pub const fn new(original_frequency: u32) -> Self {
        let multipier: u8 = {
            let ratio = (UPSAMPLE_TARGET_FREQUENCY / original_frequency) as u8;
            if ratio < 1 { 1 } else { ratio }
        };

        Self { multipier }
    }

    #[inline]
    pub fn submit(&self, window: Arc<[f32]>, start: usize) -> impl Iterator<Item = f32> {
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
                        k as f32 / self.multipier as f32,
                    ) {
                        Ok(value) => value,
                        Err(e) => panic!("{e:?}")
                    }
                }
            })
    }
}
