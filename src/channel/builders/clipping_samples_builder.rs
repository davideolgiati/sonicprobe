use crate::channel::builders::ClippingSamplesBuilder;

impl ClippingSamplesBuilder {
        pub fn new() -> ClippingSamplesBuilder {
                ClippingSamplesBuilder {
                        count: 0
                }
        }

        #[inline]
        pub fn add(&mut self, sample: f32) {
                if is_clipping(sample) {
                        self.count += 1;
                }
        }

        pub fn build(self) -> i32 {
                self.count
        }
}

const CLIP_THRESH: f32 = 0.999_999;

#[inline]
pub fn is_clipping(sample: f32) -> bool {
    sample >= CLIP_THRESH || sample <= -CLIP_THRESH
}