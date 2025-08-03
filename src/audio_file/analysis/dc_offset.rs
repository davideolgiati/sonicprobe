use std::process;

impl super::DCOffset {
    pub fn new(count: u64) -> super::DCOffset {
        super::DCOffset {
            partials: Vec::new(),
            count,
        }
    }

    #[inline]
    pub fn add(&mut self, value: f32) {
        let mut current = value as f64;
        let mut index: usize = 0;

        for mut partial in self.partials.clone() {
            if current.abs() < partial.abs() {
                (current, partial) = (partial, current)
            }

            let high = current + partial;
            let low = partial - (high - current);

            if low != 0.0 {
                match self.partials.get_mut(index) {
                    Some(value) => *value = low,
                    None => {
                        println!("error: dc offset can't update partials at index {index}");
                        process::exit(1);
                    }
                };
                index += 1;
            }
            current = high
        }

        self.partials.truncate(index);
        self.partials.push(current)
    }

    pub fn build(self) -> f32 {
        if self.count == 0 {
            return 0.0f32;
        }

        let sum: f64 = self.partials.iter().sum();

        (sum / self.count as f64) as f32
    }
}
