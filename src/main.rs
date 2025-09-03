mod dsp;
mod ui;
mod model;
mod floating_point_math;

use claxon::FlacReader;
use std::{env, process};

use crate::model::builders::audio_file_builder::audio_file_form_stream;
use crate::model::builders::cli_args_builder::cli_args_from_args_array;
use crate::model::cli_args::CliArgs;
use crate::model::output_format::OutputFormat;
use crate::ui::print_file_details;

fn main() {
    let cli_input: Vec<String> = env::args().collect();

    let args: CliArgs = match cli_args_from_args_array(&cli_input){
        Ok(value) => value,
        Err(e) => {
            println!("{e:?}");
            process::exit(1);
        }
    };

    let flac_details = match FlacReader::open(args.file_path()) {
        Ok(stream) => audio_file_form_stream(stream),
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
