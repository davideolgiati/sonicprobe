mod ui;
mod channel;
mod cli_args;
mod flac_file;
mod audio_utils;
mod output_format;
mod circular_buffer;
mod stereo_correlation_builder;
mod true_bit_depth_builder;

extern crate flac;

use std::env;
use std::fs::File;
use flac::StreamReader;

use crate::cli_args::CliArgs;
use crate::flac_file::FlacFile;
use crate::output_format::OutputFormat;
use crate::ui::print_file_details;

fn main() {
    let cli_input: Vec<String> = env::args().collect();
    let args: CliArgs = CliArgs::new(&cli_input);

    let flac_details = match StreamReader::<File>::from_file(args.file_path()) {
        Ok(stream) => FlacFile::new(stream),
        Err(error)     => panic!("error: {:?}", error),
    };

    if *args.output_format() == OutputFormat::Json {
        println!("{}", flac_details.to_json_string())
    } else {
        print_file_details(args.file_path(), &flac_details);
    }
}
