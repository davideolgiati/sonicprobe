use std::{cmp::Reverse, collections::BinaryHeap};
use ordered_float::NotNan;

type MinNonNan = Reverse<NotNan<f32>>;

pub struct RMSBuilder {
        accumulator: BinaryHeap<MinNonNan>
}

impl RMSBuilder {
        pub fn new() -> RMSBuilder {
                RMSBuilder {
                        accumulator: BinaryHeap::new()
                }
        }
        pub fn add(&mut self, value: f32) {
                let processed_value = Reverse(NotNan::new(value.powi(2)).expect("NaN not allowed"));
                self.accumulator.push(processed_value);
        }

        pub fn build(&self) -> f32 {
                let size = self.accumulator.len() as f32;
                let sum: f32 = self.accumulator.iter().map(|value: &MinNonNan| value.0.into_inner()).sum();
                let avg: f32 = sum / size;

                avg.sqrt()
        }
}