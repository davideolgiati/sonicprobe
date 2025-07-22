use crate::channel::builders::ZeroCrossingRateBuilder;

impl ZeroCrossingRateBuilder {
        pub fn new(total_samples: u64) -> ZeroCrossingRateBuilder {
                ZeroCrossingRateBuilder {
                        count: 0,
                        total_samples,
                        current_sign: 0
                }
        }

        #[inline]
        pub fn add(&mut self, value: f32) {
                let value_sign = sign(value);
                let diff = value_sign - self.current_sign;
                self.current_sign = value_sign;

                self.count += diff as u64;
        }

        pub fn build(&self) -> f32 {
                (self.count as f64 / self.total_samples as f64) as f32
        }
}

#[inline]
fn sign(value: f32) -> i8 {
        if value < 0.0 {
                return 0
        }

        1
}