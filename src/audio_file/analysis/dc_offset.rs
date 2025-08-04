use std::{mem, process};

impl super::DCOffset {
    pub const fn new(count: u64) -> Self {
        Self {
            partials: Vec::new(),
            count,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f64) {
        let mut current = value;
        let mut index: usize = 0;

        for mut partial in self.partials.clone() {
            if current.abs() < partial.abs() {
                mem::swap(&mut current, &mut partial);
            }

            let high = current + partial;
            let low = partial - (high - current);

            if low != 0.0 {
                if let Some(to_swap) = self.partials.get_mut(index) {
                    *to_swap = low;
                } else {
                    println!("error: dc offset can't update partials at index {index}");
                    process::exit(1);
                };
                index += 1;
            }
            current = high;
        }

        self.partials.truncate(index);
        self.partials.push(current);
    }

    pub fn build(self) -> f64 {
        if self.count == 0 {
            return 0.0f64;
        }

        let sum: f64 = self.partials.iter().sum();

        sum / self.count as f64
    }
}
