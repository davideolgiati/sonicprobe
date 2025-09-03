use core::f64;

use crate::{
    floating_point_math::floating_point_utils::map_sum_lossless,
    model::{decibel::Decibel, frequency::Frequency, Signal},
};

#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn calculate_dynamic_range(samples: &Signal, sample_rate: Frequency) -> Decibel {
    let chunk_size = get_chunk_size(sample_rate);
    let target_population = ((samples.len() / chunk_size) * 20) / 100;

    let mut quietest: Vec<f64> = vec![f64::MAX; target_population];
    let mut loudest: Vec<f64> = vec![f64::MIN; target_population];

    let rms_closure = get_rms_fn(chunk_size);

    for current_chunk in samples.chunks(chunk_size) {
        let current_rms = rms_closure(current_chunk);

        if quietest[target_population - 1] > current_rms {
            quietest[target_population - 1] = current_rms;
            sort_array(&mut quietest, |a, b| a < b);
        }

        if loudest[target_population - 1] < current_rms {
            loudest[target_population - 1] = current_rms;
            sort_array(&mut loudest, |a, b| a > b);
        }
    }

    let loudest_avg = loudest.iter().sum::<f64>() / target_population as f64;
    let quietest_avg = quietest.iter().sum::<f64>() / target_population as f64;

    Decibel::new(loudest_avg / quietest_avg)
}

#[allow(clippy::cast_precision_loss)]
fn get_rms_fn(chunk_size: usize) -> impl Fn(&[f64]) -> f64 {
    move |chunk: &[f64]| (map_sum_lossless(chunk, |x: f64| x.powi(2)) / chunk_size as f64).sqrt()
}

const fn get_chunk_size(sample_rate: Frequency) -> usize {
    sample_rate.to_hz() as usize * 3
}

fn sort_array<T: Fn(f64, f64) -> bool>(array: &mut [f64], cmp_fn: T) {
    let mut current = array.len() - 1;
    while current >= 1 && cmp_fn(array[current], array[current - 1]) {
        array.swap(current, current - 1);
        current -= 1;
    }
}
