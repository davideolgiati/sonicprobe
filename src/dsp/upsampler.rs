use crate::{audio_file::Signal, audio_utils::catmull_rom_interpolation, constants::UPSAMPLE_TARGET_FREQUENCY, dsp::Upsampler};

impl Upsampler {
    pub const fn new(original_frequency: u32) -> Self {
        let multipier: u8 = {
            let ratio = (UPSAMPLE_TARGET_FREQUENCY / original_frequency) as u8;
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
