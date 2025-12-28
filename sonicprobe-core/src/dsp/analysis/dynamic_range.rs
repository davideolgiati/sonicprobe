use core::f64;

use crate::{
    floating_point_math::floating_point_utils::map_sum_lossless,
    model::{decibel::Decibel, frequency::Frequency, Signal},
};

#[inline]
pub fn calculate_dynamic_range(samples: &Signal, sample_rate: Frequency) -> Decibel {
    let chunk_size = get_chunk_size(sample_rate);
    let target_population = ((samples.len() / chunk_size) * 20) / 100;

    let mut quietest: Vec<f64> = vec![f64::MAX; target_population];
    let mut loudest: Vec<f64> = vec![f64::MIN; target_population];

    let rms_closure = get_rms_fn(chunk_size);

    for current_chunk in samples.chunks(chunk_size) {
        let new_rms = rms_closure(current_chunk);

        insert_quiet_rms(&new_rms, &mut quietest);
        insert_loud_rms(&new_rms, &mut loudest);
    }

    let loudest_avg = loudest.iter().sum::<f64>() / target_population as f64;
    let quietest_avg = quietest.iter().sum::<f64>() / target_population as f64;

    Decibel::new(loudest_avg / quietest_avg)
}

fn insert_quiet_rms(new_rms: &f64, quiet_array: &mut Vec<f64>) {
    let array_size = quiet_array.len();
    let last_element_index = array_size -1;
    
    if *new_rms > quiet_array[last_element_index] {
        return
    }

    let mut to_insert = *new_rms;

    for index in 0..array_size {
        if to_insert < quiet_array[index] {
            let tmp = quiet_array[index];
            quiet_array[index] = to_insert;
            to_insert = tmp
        }
    }
}

fn insert_loud_rms(new_rms: &f64, loud_array: &mut Vec<f64>) {
    let array_size = loud_array.len();
    let last_element_index = array_size -1;
    
    if *new_rms < loud_array[last_element_index] {
        return
    }

    let mut to_insert = *new_rms;

    for index in 0..array_size {
        if to_insert > loud_array[index] {
            let tmp = loud_array[index];
            loud_array[index] = to_insert;
            to_insert = tmp
        }
    }
}

fn get_rms_fn(chunk_size: usize) -> impl Fn(&[f64]) -> f64 {
    move |chunk: &[f64]| (map_sum_lossless(chunk, |x: f64| x.powi(2)) / chunk_size as f64).sqrt()
}

const fn get_chunk_size(sample_rate: Frequency) -> usize {
    sample_rate.to_hz() * 3
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn insert_quiet_rms_insert_empty_array() {
        let mut rng = rand::rng();
        let mut quiet_array = vec![f64::MAX; 10];

        let new_rms: f64 = rng.random_range(0.0..0.9);
        insert_quiet_rms(&new_rms, &mut quiet_array);

        assert_eq!(new_rms, quiet_array[0]);
        assert_eq!(quiet_array.len(), 10);
        for index in 1..9 {
            assert_eq!(quiet_array[index], f64::MAX, "Position #{} has changed", index)
        }
    }

    #[test]
    fn insert_quiet_rms_insert_start() {
        let mut rng = rand::rng();
        let mut quiet_array = {
            let mut res: Vec<f64> = vec![];
            for _ in 0..10 {
                res.push(rng.random_range(1.0..2.0))
            }

            res.sort_by(|a, b| a.partial_cmp(b).unwrap());

            res
        };

        let new_rms: f64 = rng.random_range(0.0..0.9);
        let expected_new_lst = quiet_array[8];
        insert_quiet_rms(&new_rms, &mut quiet_array);

        assert_eq!(new_rms, quiet_array[0]);
        assert_eq!(quiet_array.len(), 10);
        assert_eq!(quiet_array[9], expected_new_lst)
    }

    #[test]
    fn insert_quiet_rms_insert_end() {
        let mut rng = rand::rng();
        let mut quiet_array = {
            let mut res: Vec<f64> = vec![];
            for _ in 0..10 {
                res.push(rng.random_range(1.0..2.0))
            }

            res.sort_by(|a, b| a.partial_cmp(b).unwrap());

            res
        };

        let new_rms: f64 = rng.random_range(quiet_array[8]..quiet_array[9]);
        let expected_unchanged: Vec<f64> = quiet_array.clone()[0..8].to_vec();
        insert_quiet_rms(&new_rms, &mut quiet_array);

        assert_eq!(new_rms, quiet_array[9]);
        assert_eq!(quiet_array.len(), 10);
        for index in 0..8 {
            assert_eq!(quiet_array[index], expected_unchanged[index], "Position #{} has changed", index)
        }
    }

    #[test]
    fn insert_quiet_rms_insert_middle() {
        let mut rng = rand::rng();
        let mut quiet_array = {
            let mut res: Vec<f64> = vec![];
            for _ in 0..10 {
                res.push(rng.random_range(1.0..2.0))
            }

            res.sort_by(|a, b| a.partial_cmp(b).unwrap());

            res
        };

        let new_rms: f64 = rng.random_range(quiet_array[4]..quiet_array[5]);
        let expected_unchanged_pre: Vec<f64> = quiet_array.clone()[0..4].to_vec();
        let expected_unchanged_post: Vec<f64> = quiet_array.clone()[5..8].to_vec();
        insert_quiet_rms(&new_rms, &mut quiet_array);

        assert_eq!(new_rms, quiet_array[5]);
        assert_eq!(quiet_array.len(), 10);
        for index in 0..4 {
            assert_eq!(quiet_array[index], expected_unchanged_pre[index], "Position #{} has changed", index)
        }
        for index in 6..9 {
            assert_eq!(quiet_array[index], expected_unchanged_post[index - 6], "Position #{} has changed", index)
        }
    }

    #[test]
    fn insert_loud_rms_insert_empty_array() {
        let mut rng = rand::rng();
        let mut loud_array = vec![f64::MIN; 10];

        let new_rms: f64 = rng.random_range(0.0..0.9);
        insert_loud_rms(&new_rms, &mut loud_array);

        assert_eq!(new_rms, loud_array[0]);
        assert_eq!(loud_array.len(), 10);
        for index in 1..9 {
            assert_eq!(loud_array[index], f64::MIN, "Position #{} has changed", index)
        }
    }

        #[test]
    fn insert_loud_rms_insert_start() {
        let mut rng = rand::rng();
        let mut loud_array = {
            let mut res: Vec<f64> = vec![];
            for _ in 0..10 {
                res.push(rng.random_range(1.0..2.0))
            }

            res.sort_by(|a, b| b.partial_cmp(a).unwrap());

            res
        };

        let new_rms: f64 = rng.random_range(2.0..3.0);
        let expected_new_lst = loud_array[8];
        insert_loud_rms(&new_rms, &mut loud_array);

        assert_eq!(new_rms, loud_array[0]);
        assert_eq!(loud_array.len(), 10);
        assert_eq!(loud_array[9], expected_new_lst)
    }

        #[test]
    fn insert_loud_rms_insert_end() {
        let mut rng = rand::rng();
        let mut loud_array = {
            let mut res: Vec<f64> = vec![];
            for _ in 0..10 {
                res.push(rng.random_range(1.0..2.0))
            }

            res.sort_by(|a, b| b.partial_cmp(a).unwrap());

            res
        };

        let new_rms: f64 = rng.random_range(loud_array[9]..loud_array[8]);
        let expected_unchanged: Vec<f64> = loud_array.clone()[0..8].to_vec();
        insert_loud_rms(&new_rms, &mut loud_array);

        assert_eq!(new_rms, loud_array[9]);
        assert_eq!(loud_array.len(), 10);
        for index in 0..8 {
            assert_eq!(loud_array[index], expected_unchanged[index], "Position #{} has changed", index)
        }
    }

        #[test]
    fn insert_loud_rms_insert_middle() {
        let mut rng = rand::rng();
        let mut loud_array = {
            let mut res: Vec<f64> = vec![];
            for _ in 0..10 {
                res.push(rng.random_range(1.0..2.0))
            }

            res.sort_by(|a, b| b.partial_cmp(a).unwrap());

            res
        };

        let new_rms: f64 = rng.random_range(loud_array[5]..loud_array[4]);
        let expected_unchanged_pre: Vec<f64> = loud_array.clone()[0..4].to_vec();
        let expected_unchanged_post: Vec<f64> = loud_array.clone()[5..8].to_vec();
        insert_loud_rms(&new_rms, &mut loud_array);

        assert_eq!(new_rms, loud_array[5]);
        assert_eq!(loud_array.len(), 10);
        for index in 0..4 {
            assert_eq!(loud_array[index], expected_unchanged_pre[index], "Position #{} has changed", index)
        }
        for index in 6..9 {
            assert_eq!(loud_array[index], expected_unchanged_post[index - 6], "Position #{} has changed", index)
        }
    }
}

