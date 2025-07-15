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