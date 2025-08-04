use std::{fs, path::Path, process};

use crate::constants::UNITS;

fn format_file_size(bytes: u64) -> String {
    let unit_index = {
        let upper_limit = UNITS.len() - 1;
        let index = match usize::try_from(bytes / 1024) {
            Ok(value) => value,
            Err(_) => upper_limit
        };

        if bytes < 1024 {
            0usize
        } else if index > upper_limit {
            upper_limit
        } else {
            index
        }
    };

    let size = match u64::try_from(unit_index) {
        Ok(value) => bytes - value * 1024,
        Err(e) => panic!("{e:?}")
    };

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
