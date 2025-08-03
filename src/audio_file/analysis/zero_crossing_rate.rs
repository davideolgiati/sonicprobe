use std::sync::Arc;

impl super::ZeroCrossingRate {
    #[inline]
    pub fn process(samples: &Arc<[f32]>, duration: f32) -> f32 {
        samples
            .windows(2)
            .map(|slice| {
                if get_value_sign(slice[0]) != get_value_sign(slice[1]) {
                    1.0
                } else {
                    0.0
                }
            })
            .sum::<f32>() / duration
    }
}

fn get_value_sign(value: f32) -> i8 {
    if value < 0.0 {
        return -1;
    }

    1
}
