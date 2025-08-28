use core::f64;

use crate::{
    floating_point_math::floating_point_utils::map_sum_lossless,
    model::{Signal, frequency::Frequency},
};

#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn calculate_dynamic_range(samples: &Signal, sample_rate: Frequency) -> f64 {
    let chunk_size = get_chunk_size(sample_rate);
    let target_population = ((samples.len() / chunk_size) * 20) / 100;

    let mut quietest: Vec<f64> = vec![f64::MAX; target_population];
    let mut loudest: Vec<f64> = vec![f64::MIN; target_population];
    let mut count = 0usize;

    for current_chunk in samples.chunks(chunk_size) {
        let current_rms =
            (map_sum_lossless(current_chunk, |x| x.powi(2)) / chunk_size as f64).sqrt();

        if count < target_population {
            loudest[target_population - 1] = current_rms;
            sort_array(&mut loudest, |a, b| a > b);
            quietest[target_population - 1] = current_rms;
            sort_array(&mut quietest, |a, b| a < b);
            count += 1;
            continue;
        }

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

    loudest_avg / quietest_avg
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
