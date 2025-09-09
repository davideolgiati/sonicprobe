#[inline]
pub fn map_sum_lossless<T: Fn(f64) -> f64>(list: &[f64], map_fn: T) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;
    
    for &current_value in list {
        let mapped_value = map_fn(current_value);
        compensation = ((sum + (mapped_value - compensation)) - sum) - (mapped_value - compensation);
        sum += mapped_value - compensation;
    }

    sum
}

