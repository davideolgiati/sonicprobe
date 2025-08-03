pub fn format_db(value: f64) -> String {
    if value > 0.0 {
        format!("+{:.2}", value)
    } else if value == 0.0 {
        String::from("0.00")
    } else {
        format!("{:.2}", value)
    }
}

pub fn format_volt(value: f32) -> String {
    if value > 0.0 {
        format!("+{:.5}", value)
    } else if value == 0.0 {
        String::from("0.00000")
    } else {
        format!("{:.5}", value)
    }
}

pub fn format_hz(value: u32) -> String {
    format!("{}", value)
}

pub fn format_dr(value: u32) -> String {
    format!("{}", value)
}
