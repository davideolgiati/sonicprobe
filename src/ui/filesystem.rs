use std::{fs, path::Path, process};

use crate::constants::UNITS;

fn format_file_size(bytes: u64) -> String {
    let unit_index = {
        let upper_limit = UNITS.len() - 1;
        let index = usize::try_from(bytes / 1024).map_or(upper_limit, |value| value);

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
        Err(e) => panic!("{e:?}"),
    };

    let unit = match UNITS.get(unit_index) {
        Some(&value) => value,
        None => {
            println!("error: filsystem index {unit_index} is not valid");
            process::exit(1);
        }
    };

    if unit_index == 0 {
        format!("{bytes} {unit}")
    } else {
        format!("{size:.1} {unit}")
    }
}

pub fn get_formatted_file_size<P: AsRef<Path>>(path: P) -> String {
    fs::metadata(path).map_or_else(
        |_| "-".to_owned(),
        |metadata| {
            let size = metadata.len();
            format_file_size(size)
        },
    )
}

pub fn filename_from_path(filepath: &str) -> Option<String> {
    Path::new(filepath)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(std::borrow::ToOwned::to_owned)
}
