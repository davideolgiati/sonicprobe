mod audio_file;
mod audio_utils;
mod cli_args;
mod dsp;
mod output_format;
mod ui;
mod model;

use claxon::FlacReader;
use std::{env, process};

use crate::cli_args::CliArgs;
use crate::model::audio_file::AudioFile;
use crate::output_format::OutputFormat;
use crate::ui::print_file_details;

fn main() {
    let cli_input: Vec<String> = env::args().collect();
    let args: CliArgs = CliArgs::new(&cli_input);

    let flac_details = match FlacReader::open(args.file_path()) {
        Ok(stream) => AudioFile::new(stream),
        Err(error) => {
            println!("error while opening {} : {:?}", args.file_path(), error);
            process::exit(1);
        }
    };

    match flac_details {
        Ok(audio_file) => {
            if *args.output_format() == OutputFormat::Json {
                println!("{}", audio_file.to_json());
            } else {
                print_file_details(args.file_path(), &audio_file);
            }
        }
        Err(e) => {
            println!("{e:?}");
            process::exit(1);
        }
    }
}
