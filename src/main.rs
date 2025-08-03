mod audio_utils;
mod cli_args;
mod dsp;
mod audio_file;
mod output_format;
mod ui;
mod constants;

extern crate flac;

use flac::StreamReader;
use std::env;
use std::fs::File;

use crate::cli_args::CliArgs;
use crate::audio_file::AudioFile;
use crate::output_format::OutputFormat;
use crate::ui::print_file_details;

fn main() {
    let cli_input: Vec<String> = env::args().collect();
    let args: CliArgs = CliArgs::new(&cli_input);

    let flac_details = match StreamReader::<File>::from_file(args.file_path()) {
        Ok(stream) => AudioFile::new(stream),
        Err(error) => panic!("error: {:?}", error),
    };

    if *args.output_format() == OutputFormat::Json {
        println!("{}", flac_details.to_json_string())
    } else {
        print_file_details(args.file_path(), &flac_details);
    }
}
