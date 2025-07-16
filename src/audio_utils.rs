use std::f64;

const CLIP_THRESH: f64 = 0.999_999;

pub fn is_clipping(sample: f64) -> bool {
        sample >= CLIP_THRESH || sample <= -CLIP_THRESH
}

pub fn to_dbfs(rms: f64) -> f64 {
        20.0 * rms.log10()
}

pub fn cubic_interpolation(y_minus1: f64, y0: f64, y1: f64, y2: f64, t: f64) -> f64 {
        let term1 = y_minus1 * ((-t)*(1.0 - t)*(2.0 - t)) / 6.0;
        let term2 = y0 * ((t + 1.0)*(1.0 - t)*(2.0 - t)) / 2.0;
        let term3 = y1 * ((t + 1.0)*t*(2.0 - t)) / 2.0;
        let term4 = y2 * ((t + 1.0)*t*(1.0 - t)) / 6.0;
        
        term1 + term2 + term3 + term4
}

pub fn low_pass_filter(cutoff_hz: f64, original_frequency: f64, numtaps: u16) -> Vec<f64> {
    // TODO: da verificare, l'ho implementato di fretta, potrebbero essere dei file ???
    let fc: f64 = cutoff_hz / original_frequency;
    let middle_point: u16 = numtaps - 1;
    let mut h: Vec<f64> = Vec::new();

    for n in 1..numtaps{
        let distance_from_center = (n - middle_point) as f64;

        if n == middle_point / 2 {
            h.push(2.0 * fc)
        } else {
            h.push((2.0 * f64::consts::PI * fc * (distance_from_center/ 2.0)) / (f64::consts::PI * (distance_from_center / 2.0)).sin())
        }

        h[n as usize] *= (0.54 - 0.46 * ((2.0 * f64::consts::PI * n as f64) / middle_point as f64)).cos()
    }

    
    let sum = h.iter().sum::<f64>();
    
    h.iter()
        .map(|sample| sample / sum)
        .collect()
}