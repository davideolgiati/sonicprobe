pub fn format_db(value: f32) -> String {
    if value > 0.0 {
        format!("+{:.2} dB", value)
    } else if value == 0.0 {
        "0.00 dB".to_string()
    } else {
        format!("{:.2} dB", value)
    }
}

pub fn format_volt(value: f32) -> String {
    if value > 0.0 {
        format!("+{:.5}  V", value)
    } else if value == 0.0 {
        "0.00000  V".to_string()
    } else {
        format!("{:.5}  V", value)
    }
}

pub fn format_hz(value: u32) -> String {
    format!("{} Hz", value)
}

pub fn format_dr(value: u32) -> String {
    format!("{} DR", value)
}
