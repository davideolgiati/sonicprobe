use std::process;

use crate::audio_file::Signal;

impl super::ZeroCrossingRate {
    #[inline]
    pub fn process(samples: &Signal, duration: f32) -> f32 {
        samples
            .windows(2)
            .map(|slice| {
                let Some(&first_sample) = slice.first() else {
                    println!("error: zero crossing rate can't get first sample");
                    process::exit(1);
                };

                let Some(&second_sample) = slice.last() else {
                    println!("error: zero crossing rate can't get second sample");
                    process::exit(1);
                };

                if get_value_sign(first_sample) == get_value_sign(second_sample) {
                    0.0
                } else {
                    1.0
                }
            })
            .sum::<f32>()
            / duration
    }
}

fn get_value_sign(value: f64) -> i8 {
    if value < 0.0 {
        return -1;
    }

    1
}
