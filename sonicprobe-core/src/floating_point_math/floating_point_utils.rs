/// Number of independent Kahan accumulators run in parallel.
///
/// Each lane carries its own running sum + compensation, so consecutive
/// samples no longer feed the same dependency chain. This exposes
/// instruction-level parallelism (the CPU pipelines the lanes) and lets the
/// backend pack the per-lane work into SIMD, instead of one scalar FP add
/// chain stalling on its own multi-cycle latency.
const SUM_LANES: usize = 8;

/// Compensated (Kahan) sum of `map_fn` applied to every element, split across
/// [`SUM_LANES`] parallel accumulators and folded once at the end.
///
/// Numerically equivalent to a single Kahan pass to ~machine epsilon; results
/// are not bit-identical to a sequential sum because the additions are
/// reordered and floating-point addition is non-associative.
#[inline]
pub fn map_sum_lossless<T: Fn(f64) -> f64>(list: &[f64], map_fn: T) -> f64 {
    let mut sum = [0.0f64; SUM_LANES];
    let mut compensation = [0.0f64; SUM_LANES];

    // `as_chunks` yields `&[f64; SUM_LANES]` fixed arrays, so the inner index
    // carries no bounds check and the lane loop unrolls cleanly.
    let (chunks, remainder) = list.as_chunks::<SUM_LANES>();

    for chunk in chunks {
        for lane in 0..SUM_LANES {
            let mapped_value = map_fn(chunk[lane]);
            let y = mapped_value - compensation[lane];
            let t = sum[lane] + y;
            compensation[lane] = (t - sum[lane]) - y;
            sum[lane] = t;
        }
    }

    // Fold the lanes back together with one more Kahan pass, carrying each
    // lane's leftover compensation into its contribution.
    let mut total = 0.0;
    let mut total_compensation = 0.0;
    for lane in 0..SUM_LANES {
        let lane_value = sum[lane] - compensation[lane];
        let y = lane_value - total_compensation;
        let t = total + y;
        total_compensation = (t - total) - y;
        total = t;
    }

    // Tail elements that didn't fill a full lane-width chunk.
    for &current_value in remainder {
        let mapped_value = map_fn(current_value);
        let y = mapped_value - total_compensation;
        let t = total + y;
        total_compensation = (t - total) - y;
        total = t;
    }

    total
}

