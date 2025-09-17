use crate::model::Signal;

/// Given a `mono_signal`, loops over each sample and looks for the ones greater
/// or equal to 1.0 or lower or equal to -1.0.
/// 
/// Returns `total_clipping_samples` an u64 varaible containing all distorted 
/// samples found while looping throught the signal.
/// 
/// This function has no side effects.
/// This function is declared as `#[inline]`
///
/// # Examples
/// 
/// ```
///     let mut rng = rand::rng();
///     let samples = (0..10).map(|_| rng.random_range(-0.99..0.99)).collect();
///     let res = count_clipping_samples(&samples)
/// ```
///
#[inline]
#[must_use] pub fn count_clipping_samples(mono_signal: &Signal) -> u64 {
    let mut total_clipping_samples = 0u64;
    for &sample in mono_signal.iter() {
        if is_distorted(sample) {
            total_clipping_samples += 1;
        }
    }

    total_clipping_samples
}

/// Given a `sample` returns true if his value is greater or equal to 1.0 or 
/// lower or equal to -1.0.
/// 
/// This function has no side effects.
/// This function is declared as `#[inline]`
///
/// # Examples
/// 
/// ```
///     let sample: f64 = 1.0;
///     let res = is_distorted(sample)
///     assert_eq!(res, true)
/// ```
///
#[inline]
#[must_use] pub const fn is_distorted(sample: f64) -> bool {
    sample >= 1.0 || sample <= -1.0
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use rand::Rng;

    use crate::dsp::analysis::clipping::count_clipping_samples;

    #[test]
    fn no_clip() {
        let mut rng = rand::rng();
        let samples = (0..10).map(|_| rng.random_range(-0.99..0.99)).collect();
        let res = count_clipping_samples(&samples);

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
        let res = count_clipping_samples(&samples);

        assert_eq!(res, 4u64);
    }

    #[test]
    fn all_clip() {
        let samples = (0..10)
            .map(|i| if i % 2 == 0 { -1.0 } else { 1.0 })
            .collect();
        let res = count_clipping_samples(&samples);

        assert_eq!(res, 10u64);
    }
}
