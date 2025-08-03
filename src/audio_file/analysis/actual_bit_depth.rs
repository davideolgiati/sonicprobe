use std::sync::Arc;

use crate::{constants::{MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, MAX_8_BIT}};

impl super::ActualBitDepth {
    #[inline]
    pub fn process(signal: Arc<[f32]>, reported_depth: u8) -> u8 {
        let factor = match reported_depth {
            8 => MAX_8_BIT,
            16 => MAX_16_BIT,
            24 => MAX_24_BIT,
            32 => MAX_32_BIT,
            _ => panic!("Unknown bit depth"),
        };

        signal
            .iter()
            .map(|sample| {
                if *sample == 0.0 {
                    return 0u8;
                }

                let trailing_zeros = ((*sample * factor) as i32).trailing_zeros();
                (reported_depth as u32 - trailing_zeros) as u8
            })
            .take_while(|x| *x < reported_depth)
            .max_by(|a, b| a.cmp(b))
            .unwrap()
    }
}
