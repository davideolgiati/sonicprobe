use rayon::prelude::*;

pub struct TrueBitDepthBuilder {
    min: u8,
    max: u8,
    avarage: f32,
    reported_depth: u8,
    sample_count: u64,
}

impl TrueBitDepthBuilder {
    pub fn new(depth: u8, sample_count: u64) -> TrueBitDepthBuilder {
        TrueBitDepthBuilder {
            min: u8::MAX,
            max: u8::MIN,
            avarage: 0.0,
            reported_depth: depth,
            sample_count,
        }
    }

    #[inline]
    pub fn add(&mut self, mapped_stream: Vec<f32>, factor: f32) {
        let mut real_depths = mapped_stream
                .par_iter()
                .map(|sample| (*sample * factor) as i32)
                .map(|sample| self.reported_depth - trailing_zeroes(sample))
                .collect::<Vec<u8>>();

        real_depths.sort();

        self.min = real_depths[0];
        self.max = real_depths[real_depths.len() - 1];
        self.avarage = (real_depths.par_iter().map(|s| *s as u64).sum::<u64>() as f64 / (self.sample_count * 2) as f64) as f32

    }

    pub fn build(&self) -> (u8, u8, u8) {
        (
                self.min,
                self.max,
                self.avarage.round() as u8
        )
    }
}

fn trailing_zeroes(input: i32) -> u8 {
    let mut bits: u8 = 0;
    let mut data = input;

    if data != 0 {
        if (data & 0x0000FFFF) == 0 {
            bits += 16;
            data >>= 16;
        }
        if (data & 0x000000FF) == 0 {
            bits += 8;
            data >>= 8;
        }
        if (data & 0x0000000F) == 0 {
            bits += 4;
            data >>= 4;
        }
        if (data & 0x00000003) == 0 {
            bits += 2;
            data >>= 2;
        }
        bits += ((data & 1) ^ 1) as u8;
    }
    
    bits
}
