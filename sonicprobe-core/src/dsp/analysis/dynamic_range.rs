use core::f64;
use std::mem;

use crate::{
    model::{decibel::Decibel, frequency::Frequency}, sonicprobe_error::SonicProbeError
};

const TARGET_SAMPLE_POPULATION_SHARE: usize = 20;

pub struct DynamicRangeMeter {
    chunk_square_sum: f64,
    samples_in_chunk: usize,
    chunk_size: usize,
    quiet_parts_rms: Vec<f64>,
    loud_parts_rms: Vec<f64>
}

impl DynamicRangeMeter {
    pub fn new(samples_count: &usize, sample_rate: &Frequency) -> DynamicRangeMeter {
        let chunk_size = get_chunk_size(*sample_rate);
        let target_population = get_target_population_count(samples_count, &chunk_size);

        Self {
            chunk_square_sum: 0.0,
            samples_in_chunk: 0,
            chunk_size,
            quiet_parts_rms: vec![f64::MAX; target_population],
            loud_parts_rms: vec![f64::MIN; target_population]
        }
    }

    pub fn push_sample(&mut self, sample: &f64) -> Result<(), SonicProbeError>{
        // Accumulate the chunk's sum of squares incrementally — no per-sample
        // buffer is stored and the chunk is never re-scanned. A single running
        // add (vs Kahan's 4 dependent ops) keeps this per-sample path cheap.
        self.chunk_square_sum += *sample * *sample;
        self.samples_in_chunk += 1;

        if self.samples_in_chunk == self.chunk_size {
            let new_rms = (self.chunk_square_sum / self.chunk_size as f64).sqrt();

            update_quiet_rms_population(&new_rms, &mut self.quiet_parts_rms);
            update_loud_rms_population(&new_rms, &mut self.loud_parts_rms);

            self.chunk_square_sum = 0.0;
            self.samples_in_chunk = 0;
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

    fn push_all(meter: &mut DynamicRangeMeter, samples: &[f64]) {
        for sample in samples {
            meter.push_sample(sample).unwrap();
        }
    }

    // --- Full-meter behavior locks (impl-independent, survive any refactor) ---

    #[test]
    fn meter_constant_signal_gives_zero_dr() {
        let sample_rate = Frequency::CdQuality;
        let chunk = sample_rate.to_hz() * 3;
        let n = chunk * 10;
        let signal = vec![0.5f64; n];

        let mut meter = DynamicRangeMeter::new(&n, &sample_rate);
        push_all(&mut meter, &signal);

        // Every 3s chunk has identical RMS → loud_avg == quiet_avg → ratio 1 → 0 dB.
        assert!(meter.get_dr_value().get_value().abs() < 1e-9);
    }

    #[test]
    fn meter_two_level_signal_gives_expected_dr() {
        let sample_rate = Frequency::CdQuality;
        let chunk = sample_rate.to_hz() * 3;
        let n = chunk * 10;

        // 5 quiet chunks @ 0.1, 5 loud chunks @ 0.8. RMS of a constant a is |a|.
        let mut signal = vec![0.1f64; chunk * 5];
        signal.extend(std::iter::repeat(0.8f64).take(chunk * 5));
        assert_eq!(signal.len(), n);

        let mut meter = DynamicRangeMeter::new(&n, &sample_rate);
        push_all(&mut meter, &signal);

        // Top-20% population ≈ 0.8, bottom-20% ≈ 0.1 → DR = 20*log10(0.8/0.1).
        let expected = 20.0 * (0.8f64 / 0.1).log10();
        assert!((meter.get_dr_value().get_value() - expected).abs() < 1e-6);
    }

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

