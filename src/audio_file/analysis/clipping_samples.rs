use crate::audio_file::Signal;

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
    sample.abs() >= 1.0// - crate::constants::CLIP_EPSILON
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use rand::Rng;

    use crate::audio_file::analysis::ClippingSamples;

    #[test]
    fn no_clip() {
        let mut rng = rand::rng();
        let samples = (0..10).map(|_| rng.random_range(-0.99..0.99)).collect();
        let res = ClippingSamples::process(&samples);

        assert_eq!(res, 0u64);
    }

    #[test]
    fn some_clip() {
        let mut rng = rand::rng();
        let samples = (0..10)
            .map(|i| {
                if i % 3 != 0 {
                    rng.random_range(-0.99..0.99)
                } else {
                    1.0
                }
            })
            .collect();
        let res = ClippingSamples::process(&samples);

        assert_eq!(res, 4u64);
    }

    #[test]
    fn all_clip() {
        let samples = (0..10)
            .map(|i| if i % 2 == 0 { -1.0 } else { 1.0 })
            .collect();
        let res = ClippingSamples::process(&samples);

        assert_eq!(res, 10u64);
    }
}
