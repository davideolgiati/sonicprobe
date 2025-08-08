#[inline]
pub fn map_sum_lossless<T: Fn(f64) -> f64>(input: &[f64], mapping_fn: T) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;
    
    for &value in input {
        let current_value = mapping_fn(value);
        compensation = ((sum + (current_value - compensation)) - sum) - (current_value - compensation);
        sum += current_value - compensation;
    }

    sum
}