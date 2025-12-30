use core::f64;
use std::mem;

use crate::{
    analysis::root_mean_square::compute_root_mean_square, model::{decibel::Decibel, frequency::Frequency}, sonicprobe_error::SonicProbeError
};

const TARGET_SAMPLE_POPULATION_SHARE: usize = 20;

pub struct DynamicRangeMeter {
    buffer: Vec<f64>,
    buffer_size: usize,
    next_insert_index: usize,
    quiet_parts_rms: Vec<f64>,
    loud_parts_rms: Vec<f64>
}

impl DynamicRangeMeter {
    pub fn new(samples_count: &usize, sample_rate: &Frequency) -> DynamicRangeMeter {
        let buffer_size = get_chunk_size(*sample_rate);
        let target_population = get_target_population_count(samples_count, &buffer_size);

        Self {
            buffer: vec![0.0; buffer_size],
            buffer_size,
            next_insert_index: 0,
            quiet_parts_rms: vec![f64::MAX; target_population],
            loud_parts_rms: vec![f64::MIN; target_population]
        }
    }

    pub fn push_sample(&mut self, sample: &f64) -> Result<(), SonicProbeError>{
        self.buffer[self.next_insert_index] = *sample;
        self.next_insert_index += 1;

        if self.next_insert_index == self.buffer_size {
            let new_rms = compute_root_mean_square(&self.buffer)?;

            update_quiet_rms_population(&new_rms, &mut self.quiet_parts_rms);
            update_loud_rms_population(&new_rms, &mut self.loud_parts_rms);

            self.next_insert_index = 0;
        }

        Ok(())
    }

    pub fn get_dr_value(&self) -> Decibel {
        let loudest_avg = self.loud_parts_rms.iter().sum::<f64>() / self.loud_parts_rms.len() as f64;
        let quietest_avg = self.quiet_parts_rms.iter().sum::<f64>() / self.quiet_parts_rms.len() as f64;

        Decibel::new(loudest_avg / quietest_avg)
    }
}

fn get_target_population_count(samples_count: &usize, chunks_size: &usize) -> usize {
    let chunks_in_signal = samples_count / chunks_size;
    let target_population_count = (chunks_in_signal * TARGET_SAMPLE_POPULATION_SHARE) / 100;

    target_population_count
}

fn update_quiet_rms_population(new_rms: &f64, quiet_rms_population: &mut Vec<f64>) {
    let array_size = quiet_rms_population.len();
    let loudest_element = quiet_rms_population[array_size - 1];
    
    if *new_rms > loudest_element {
        return
    }

    let mut to_insert = *new_rms;

    for index in 0..array_size {
        if to_insert < quiet_rms_population[index] {
            mem::swap(&mut quiet_rms_population[index], &mut to_insert);
        }   
    }
}

fn update_loud_rms_population(new_rms: &f64, loud_rms_population: &mut Vec<f64>) {
    let array_size = loud_rms_population.len();
    let quietest_element = loud_rms_population[array_size - 1];
    
    if *new_rms < quietest_element {
        return
    }

    let mut to_insert = *new_rms;

    for index in 0..array_size {
        if to_insert > loud_rms_population[index] {
            mem::swap(&mut loud_rms_population[index], &mut to_insert);
        }
    }
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
        update_quiet_rms_population(&new_rms, &mut quiet_array);

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
        update_quiet_rms_population(&new_rms, &mut quiet_array);

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
        update_quiet_rms_population(&new_rms, &mut quiet_array);

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
        update_quiet_rms_population(&new_rms, &mut quiet_array);

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
        update_loud_rms_population(&new_rms, &mut loud_array);

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
        update_loud_rms_population(&new_rms, &mut loud_array);

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
        update_loud_rms_population(&new_rms, &mut loud_array);

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
        update_loud_rms_population(&new_rms, &mut loud_array);

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

