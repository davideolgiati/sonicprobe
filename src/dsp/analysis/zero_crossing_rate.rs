use crate::model::{Signal, frequency::Frequency};

#[inline]
pub fn calculate_zero_crossing_rate(samples: &Signal, sample_rate: Frequency) -> u64 {
    let reminder_size = samples.len() - (samples.len() % sample_rate.to_hz() as usize);

    let mut crossing_rate = 0u64;

    for window in samples[0..reminder_size].windows(2) {
        if get_value_sign(window[0]) != get_value_sign(window[1]) {
            crossing_rate += 1;
        }
    }

    crossing_rate / (reminder_size / sample_rate.to_hz() as usize) as u64
}

fn get_value_sign(value: f64) -> i8 {
    if value < 0.0 {
        return -1;
    }

    1
}
