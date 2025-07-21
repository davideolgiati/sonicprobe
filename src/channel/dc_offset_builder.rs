pub struct DCOffsetBuilder {
        accumulator: Vec<f32>
}

impl DCOffsetBuilder {
        pub fn new() -> DCOffsetBuilder {
                DCOffsetBuilder {
                        accumulator: Vec::new()
                }
        }
        pub fn add(&mut self, value: &f32) {
                self.accumulator.push(*value);
        }

        pub fn build(&mut self) -> f32 {
                self.accumulator.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let size = self.accumulator.len() as f32;
                let sum: f32 = self.accumulator.iter().sum();
                let avg: f32 = sum / size;

                avg
        }
}