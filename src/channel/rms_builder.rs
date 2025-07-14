use std::{cmp::Reverse, collections::BinaryHeap};
use ordered_float::NotNan;

type MinNonNan = Reverse<NotNan<f64>>;

pub struct RMSBuilder {
        accumulator: BinaryHeap<MinNonNan>
}

impl RMSBuilder {
        pub fn new() -> RMSBuilder {
                RMSBuilder {
                        accumulator: BinaryHeap::new()
                }
        }
        pub fn add(&mut self, value: f64) {
                let processed_value = Reverse(NotNan::new(value.powi(2)).expect("NaN not allowed"));
                self.accumulator.push(processed_value);
        }

        pub async fn build(&self) -> f64 {
                let size = self.accumulator.len() as f64;
                let sum: f64 = self.accumulator.iter().map(|value: &MinNonNan| value.0.into_inner()).sum();
                let avg: f64 = sum / size;

                avg.sqrt()
        }
}