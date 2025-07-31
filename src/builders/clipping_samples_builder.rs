use crate::builders::ClippingSamplesBuilder;

impl ClippingSamplesBuilder {
    pub fn new() -> ClippingSamplesBuilder {
        ClippingSamplesBuilder { count: 0 }
    }

    #[inline]
    pub fn add(&mut self, sample: f32) {
        if is_clipping(sample) {
            self.count += 1;
        }
    }

    pub fn build(self) -> u32 {
        self.count
    }
}

pub fn is_clipping(sample: f32) -> bool {
    (sample.abs() - 0.95).abs() < f32::EPSILON || sample.abs() > 0.95
}
