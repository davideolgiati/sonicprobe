use crate::{audio_file::Signal, constants::{MAX_16_BIT, MAX_24_BIT, MAX_32_BIT, MAX_8_BIT}};

impl super::ActualBitDepth {
    #[inline]
    pub fn process(signal: &Signal, reported_depth: u8) -> u8 {
        let factor = match reported_depth {
            8 => MAX_8_BIT,
            16 => MAX_16_BIT,
            24 => MAX_24_BIT,
            32 => MAX_32_BIT,
            _ => panic!("Unknown bit depth"),
        };

        let res = signal
            .iter()
            .map(|sample| {
                if *sample == 0.0 {
                    return 0u8;
                }

                let trailing_zeros = ((*sample * factor).trunc() as i32).trailing_zeros();
                (reported_depth as u32 - trailing_zeros) as u8
            })
            .take_while(|x| *x < reported_depth)
            .max_by(std::cmp::Ord::cmp);

        res.map_or(reported_depth, |depth| reported_depth - depth)
            
    }
}
