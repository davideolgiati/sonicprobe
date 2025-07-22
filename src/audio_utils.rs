use std::f32;

pub fn to_dbfs(rms: f32) -> f32 {
    20.0 * rms.log10()
}

#[inline]
pub fn catmull_rom_interpolation(y0: f64, y1: f64, y2: f64, y3: f64, t: f64) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;

    0.5 * (
        2.0 * y1 +
        (-y0 + y2) * t +
        (2.0 * y0 - 5.0 * y1 + 4.0 * y2 - y3) * t2 +
        (-y0 + 3.0 * y1 - 3.0 * y2 + y3) * t3
    ) as f32
}

fn hz_to_radian(frequency: f32, sample_rate: f32) -> f32 {
    (frequency / sample_rate) * 2.0 * f32::consts::PI
}

pub fn low_pass_filter(cutoff: f32, sample_rate: f32, numtaps: i16) -> Vec<f32> {
    let center_frequency: f32 = hz_to_radian(cutoff, sample_rate);
    let window_center: i16 = (numtaps - 1) / 2;
    let window = (0..numtaps).map(
        |n| 0.54 - 0.46 * ((2.0 * f32::consts::PI * n as f32) / (numtaps - 1) as f32).cos()
    ).collect::<Vec<f32>>();

    (0..numtaps)
        .map(|n| {
            let offset = n - window_center;
            
            if offset != 0 {
                (center_frequency * offset as f32).sin() / (f32::consts::PI * offset as f32) * window[n as usize]
            } else {
                center_frequency / f32::consts::PI * window[n as usize]
            }
        })
        .collect()
}