use std::f32;

const CLIP_THRESH: f32 = 0.999_999;

pub fn is_clipping(sample: f32) -> bool {
    sample >= CLIP_THRESH || sample <= -CLIP_THRESH
}

pub fn to_dbfs(rms: f32) -> f32 {
    20.0 * rms.log10()
}

pub fn catmull_rom_interpolation(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;

    0.5 * (
        2.0 * y1 +
        (-y0 + y2) * t +
        (2.0 * y0 - 5.0 * y1 + 4.0 * y2 - y3) * t2 +
        (-y0 + 3.0 * y1 - 3.0 * y2 + y3) * t3
    )
}

fn hz_to_radian(frequency: f32, sample_rate: f32) -> f32 {
    (frequency / sample_rate) * 2.0 * f32::consts::PI
}

pub fn low_pass_filter(cutoff: f32, sample_rate: f32, numtaps: i16) -> Vec<f32> {
    let center_frequency: f32 = hz_to_radian(cutoff, sample_rate);
    let window_center: i16 = (numtaps - 1) / 2;
    let window = (0..numtaps).map(
        |n| 0.54 - 0.46 * ((2.0 * f32::consts::PI * n as f32) / (numtaps - 1) as f32).cos()
    );

    (0..numtaps)
        .map(|n| {
            let offset = n - window_center;
            
            if offset == 0 {
                center_frequency / f32::consts::PI
            } else {
                (center_frequency * offset as f32).sin() / (f32::consts::PI * offset as f32)
            }
        })
        .zip(window)
        .map(|(frequency, window)| frequency * window)
        .collect()
}