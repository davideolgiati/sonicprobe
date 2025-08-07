use std::mem;

use crate::sonicprobe_error::SonicProbeError;

#[inline]
pub fn map_sum_lossless<T: Fn(f64) -> f64>(
    values: &[f64],
    mapping_fn: T,
) -> Result<f64, SonicProbeError> {
    let mut partials: Vec<f64> = Vec::new();

    if values.is_empty() {
        return Ok(0.0f64);
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
                    return Err(SonicProbeError {
                        location: format!("{}:{}", file!(), line!()),
                        message: format!("can't update partials at index {index}"),
                    });
                };
                index += 1;
            }

            current = high;
        }

        partials.truncate(index);
        partials.push(current);
    }

    Ok(partials.iter().sum())
}
