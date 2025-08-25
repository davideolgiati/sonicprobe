use crate::model::{frequency::Frequency, Signal};

impl super::ZeroCrossingRate {
    #[inline]
    pub fn process(samples: &Signal, sample_rate: Frequency) -> u64 {
        let mut buffer = vec![0u64; sample_rate.to_hz() as usize];
        let mut buff_index = 0usize;

        let mut crossing_rates = vec![0u64; (samples.len() / buffer.len()) + 1];
        let mut cr_index = 0usize;

        for window in samples.windows(2) {
            if buff_index == buffer.len() {
                crossing_rates[cr_index] = buffer.iter().sum::<u64>();
                cr_index += 1;
                buff_index = 0;
            }
            
            if get_value_sign(window[0]) == get_value_sign(window[1]) {
                buffer[buff_index] = 0;
            } else {
                buffer[buff_index] = 1;
            }

            buff_index += 1;
        }

        crossing_rates.iter().sum::<u64>() / crossing_rates.len() as u64
    }
}

fn get_value_sign(value: f64) -> i8 {
    if value < 0.0 {
        return -1;
    }

    1
}
