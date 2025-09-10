use std::{fs, path::Path};

const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

fn format_file_size(bytes: u64) -> Result<String, String> {
    let unit_index = {
        let upper_limit = UNITS.len() - 1;
        let index = usize::try_from(bytes).map_or(upper_limit, |value| {
            let mut tmp = value;
            let mut output = 0;
            while tmp > 1024 {
                tmp /= 1024;
                output += 1;
            }
            output
        });

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
        Err(e) => return Err(format!("{e:?}")),
    };

    let Some(&unit) = UNITS.get(unit_index) else {
        return Err(format!("error: filsystem index {unit_index} is not valid"));
    };

    if unit_index == 0 {
        Ok(format!("{bytes} {unit}"))
    } else {
        Ok(format!("{size:.1} {unit}"))
    }
}

pub fn get_formatted_file_size<P: AsRef<Path>>(path: P) -> Result<String, String> {
    match fs::metadata(path) {
        Ok(metadata) => {
            let formatted = format_file_size(metadata.len())?;
            Ok(formatted)
        },
        Err(e) => Err(format!("{e:?}"))
    }
}

pub fn filename_from_path(filepath: &str) -> Option<String> {
    Path::new(filepath)
        .file_stem()
        .and_then(|name| name.to_str())
        .map(std::borrow::ToOwned::to_owned)
}
