use crate::channel::builders::RMSBuilder;

impl RMSBuilder {
        pub fn new() -> RMSBuilder {
                RMSBuilder {
                        sum: 0.0,
                        count: 0
                }
        }

        #[inline]
        pub fn add(&mut self, value: f32) {
                self.sum += (value as f64).powi(2);
                self.count += 1;
        }

        pub fn build(&self) -> f32 {
                if self.count == 0 {
                        return 0.0f32
                }

                (self.sum / self.count as f64).sqrt() as f32
        }
}