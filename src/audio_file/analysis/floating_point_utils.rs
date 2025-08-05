use std::{mem, process};

#[inline]
pub fn map_sum_lossless<T: Fn(f64) -> f64>(values: &[f64], mapping_fn: T) -> f64 {
        let mut partials: Vec<f64> = Vec::new();

        if values.is_empty() {
            return 0.0f64;
        }

        for &value in values {
                let mut current = mapping_fn(value);
                let mut index: usize = 0;
    
                for mut partial in partials.clone() {
                    if current.abs() < partial.abs() {
                        mem::swap(&mut current, &mut partial);
                    }
    
                    let high = current + partial;
                    let low = partial - (high - current);
    
                    if low != 0.0 {
                        if let Some(to_swap) = partials.get_mut(index) {
                            *to_swap = low;
                        } else {
                            println!("error: root mean square can't update partials at index {index}");
                            process::exit(1);
                        };
                        index += 1;
                    }
    
                    current = high;
                }
    
                partials.truncate(index);
                partials.push(current);
        }

        partials.iter().sum()
}