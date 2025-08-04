pub fn format_db(value: f64) -> String {
    if value > 0.0 {
        format!("+{value:.2}")
    } else if value == 0.0 {
        String::from("0.00")
    } else {
        format!("{value:.2}")
    }
}

pub fn format_volt(value: f64) -> String {
    if value > 0.0 {
        format!("+{value:.5}")
    } else if value == 0.0 {
        String::from("0.00000")
    } else {
        format!("{value:.5}")
    }
}

pub fn format_hz(value: i64) -> String {
    format!("{value}")
}

pub fn format_dr(value: i64) -> String {
    format!("{value}")
}
