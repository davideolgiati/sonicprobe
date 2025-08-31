use crate::{audio_utils::to_dbfs, model::Signal};

#[inline]
pub fn find_signal_peak(samples: &Signal) -> f64 {
    let mut peak_h = f64::MIN;
    let mut peak_l = f64::MAX;

    for &value in samples.iter() {
        if value > peak_h {
            peak_h = value;
        } else if value < peak_l {
            peak_l = value;
        }
    }

    if peak_l.abs() > peak_h {
        to_dbfs(peak_l)
    } else {
        to_dbfs(peak_h)
    }
}
