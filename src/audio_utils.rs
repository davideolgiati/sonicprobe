const CLIP_THRESH: f64 = 0.999_999;

pub fn is_clipping(sample: f64) -> bool {
        sample >= CLIP_THRESH || sample <= -CLIP_THRESH
}

pub fn to_dbfs(rms: f64) -> f64 {
        20.0 * rms.log10()
}