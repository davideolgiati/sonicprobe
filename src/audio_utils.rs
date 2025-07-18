use std::f32;

const CLIP_THRESH: f32 = 0.999_999;

pub fn is_clipping(sample: f32) -> bool {
        sample >= CLIP_THRESH || sample <= -CLIP_THRESH
}

pub fn to_dbfs(rms: f32) -> f32 {
        20.0 * rms.log10()
}

pub fn cubic_interpolation(y_minus1: f32, y0: f32, y1: f32, y2: f32, t: f32) -> f32 {
        let term1 = y_minus1 * ((-t)*(1.0 - t)*(2.0 - t)) / 6.0;
        let term2 = y0 * ((t + 1.0)*(1.0 - t)*(2.0 - t)) / 2.0;
        let term3 = y1 * ((t + 1.0)*t*(2.0 - t)) / 2.0;
        let term4 = y2 * ((t + 1.0)*t*(1.0 - t)) / 6.0;
        
        term1 + term2 + term3 + term4
}

pub fn low_pass_filter(cutoff_hz: f32, original_frequency: f32, numtaps: i16) -> Vec<f32> {
    let fc: f32 = cutoff_hz / original_frequency;
    let center: i16 = (numtaps - 1) / 2;
    let mut h: Vec<f32> = Vec::new();

    for n in 0..numtaps{
        let distance_from_center = (n - center) as f32;

        if n == center {
            h.push(2.0 * fc)
        } else {
            h.push((2.0 * f32::consts::PI * fc * distance_from_center) / (f32::consts::PI * distance_from_center).sin())
        }

        h[n as usize] *= (0.54 - 0.46 * ((2.0 * f32::consts::PI * n as f32) / (numtaps - 1) as f32)).cos()
    }

    
    let sum = h.iter().sum::<f32>();
    
    h.iter()
        .map(|sample| sample / sum)
        .collect()
}