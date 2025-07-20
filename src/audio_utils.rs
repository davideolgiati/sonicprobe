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

fn hz_to_radian(frequency: f32, sample_rate: f32) -> f32 {
    (frequency / sample_rate) * 2.0 * f32::consts::PI
}

pub fn low_pass_filter(cutoff: f32, sample_rate: f32, numtaps: i16) -> Vec<f32> {
    let center_frequency: f32 = hz_to_radian(cutoff, sample_rate);
    let window_center: i16 = (numtaps - 1) / 2;
    let window: Vec<f32> = (0..numtaps).map(
        |n| 0.54 - 0.46 * ((2.0 * f32::consts::PI * n as f32) / (numtaps - 1) as f32).cos()
    ).collect();

    let mut frequencies: Vec<f32> = Vec::new();

    for n in 0..numtaps{
        let offset = n - window_center;
        let frequency = {
            if offset == 0 {
                center_frequency / f32::consts::PI
            } else {
                (center_frequency * offset as f32).sin() / (f32::consts::PI * offset as f32)
            }
        };

        frequencies.push(frequency * window[n as usize])
    }

    frequencies
}