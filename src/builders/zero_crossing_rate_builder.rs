use crate::builders::ZeroCrossingRateBuilder;

impl ZeroCrossingRateBuilder {
    pub fn new(duration: f32) -> ZeroCrossingRateBuilder {
        ZeroCrossingRateBuilder {
            count: 0,
            duration,
            current_sign: 0,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        let value_sign = sign(value);
        let diff = {
            if value_sign != self.current_sign {
                1
            } else {
                0
            }
        };
        self.current_sign = value_sign;

        self.count += diff as u64;
    }

    pub fn build(&self) -> f32 {
        (self.count as f64 / self.duration as f64) as f32
    }
}

fn sign(value: f32) -> i8 {
    if value < 0.0 {
        return -1;
    }

    1
}
