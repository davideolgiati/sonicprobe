use std::{fs, path::Path, process};

use crate::constants::UNITS;

fn format_file_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    let unit = match UNITS.get(unit_index) {
        Some(&value) => value,
        None => {
            println!("error: filsystem index {unit_index} is not valid");
            process::exit(1);
        }
    };

    if unit_index == 0 {
        format!("{} {}", bytes, unit)
    } else {
        format!("{:.1} {}", size, unit)
    }
}

pub fn get_formatted_file_size<P: AsRef<Path>>(path: P) -> String {
    let metadata = fs::metadata(path).unwrap();
    let size = metadata.len();
    format_file_size(size)
}

pub fn filename_from_path(filepath: &str) -> String {
    Path::new(filepath)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
        .unwrap()
}
