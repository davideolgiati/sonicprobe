use crate::channel::builders::PeakBuilder;

impl PeakBuilder {
        pub fn new() -> PeakBuilder {
                PeakBuilder {
                        current_max: f32::MIN
                }
        }

        #[inline]
        pub fn add(&mut self, value: f32) {
                if value > self.current_max {
                        self.current_max = value
                }
        }

        pub fn build(self) -> f32 {
                self.current_max
        }
}