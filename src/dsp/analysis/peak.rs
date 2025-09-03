use crate::{model::{decibel::Decibel, Signal}};

#[inline]
pub fn find_signal_peak(samples: &Signal) -> Decibel {
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
        Decibel::new(peak_l.abs())
    } else {
        Decibel::new(peak_h)
    }
}
