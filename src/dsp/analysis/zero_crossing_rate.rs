use crate::model::{Signal, frequency::Frequency};

#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn calculate_zero_crossing_rate(samples: &Signal, sample_rate: Frequency) -> u64 {
    let main_section_size = samples.len() - (samples.len() % sample_rate.to_hz() as usize);
    let adjusted_reminder_crossing_rate = (samples.len() - main_section_size) as f64 / f64::from(sample_rate.to_hz());

    let mut crossing_rate = 0u64;
    let mut reminder_crossing_rate = 0.0f64;

    for window in samples[0..main_section_size].windows(2) {
        if get_value_sign(window[0]) != get_value_sign(window[1]) {
            crossing_rate += 1;
        }
    }

    for window in samples[main_section_size..].windows(2) {
        if get_value_sign(window[0]) != get_value_sign(window[1]) {
            reminder_crossing_rate += adjusted_reminder_crossing_rate;
        }
    }

    crossing_rate += reminder_crossing_rate as u64;

    crossing_rate / (1 + (main_section_size / sample_rate.to_hz() as usize) as u64)
}

const fn get_value_sign(value: f64) -> i8 {
    if value < 0.0 {
        return -1;
    }

    1
}
