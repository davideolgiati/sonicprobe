use crate::audio_file::Signal;

const CLIP_EPSILON: f64 = 1e-12;

impl super::ClippingSamples {
    #[inline]
    pub fn process(samples: &Signal) -> u64 {
        let mut count = 0u64;
        for &sample in samples.iter() {
            if is_clipping(sample) {
                count += 1;
            }
        }

        count
    }
}

#[inline]
pub fn is_clipping(sample: f64) -> bool {
    if !sample.is_finite() {
        return true;
    }

    sample.abs() > 1.0 + CLIP_EPSILON || (sample.abs() - 1.0).abs() <= CLIP_EPSILON
}
