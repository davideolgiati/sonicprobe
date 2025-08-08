use crate::audio_file::Signal;

impl super::ClippingSamples {
    #[inline]
    pub fn process(samples: &Signal) -> u64 {
        samples
            .iter()
            .filter(|&&x| is_clipping(x))
            .map(|_| 1)
            .sum()
    }
}

pub fn is_clipping(sample: f64) -> bool {
    (sample.abs() - 0.95).abs() < f64::EPSILON || sample.abs() > 0.95
}
