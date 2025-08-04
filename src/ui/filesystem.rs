use std::{fs, path::Path, process};

use crate::constants::UNITS;

fn format_file_size(bytes: u64) -> String {
    let unit_index = {
        let upper_limit = UNITS.len() - 1;
        let index = usize::try_from(bytes).map_or(
            upper_limit,
            |value| {
                let mut tmp = value;
                let mut output = 0;
                while tmp > 1024 {
                    tmp /= 1024;
                    output += 1;
                }
                output
            },
        );

        if bytes < 1024 {
            0usize
        } else if index > upper_limit {
            upper_limit
        } else {
            index
        }
    };

    let size = match u32::try_from(unit_index) {
        Ok(value) => bytes / 1024u64.pow(value),
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
        |e| panic!("{e:?}"),
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
