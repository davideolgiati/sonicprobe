use std::process;

use crate::OutputFormat;

pub struct CliArgs {
    file_path: String,
    output_format: OutputFormat,
}

impl CliArgs {
    pub fn new(args: &[String]) -> Self {
        assert!((args.len() >= 2), "No input file specified");

        let input_file = args.get(1).map_or_else(
            || {
                println!("error: no input provided :(");
                process::exit(1);
            },
            |value| value,
        );

        let output_format: OutputFormat = {
            if args.len() >= 3
                && args
                    .iter()
                    .any(|option| option.eq_ignore_ascii_case("--json"))
            {
                OutputFormat::Json
            } else {
                OutputFormat::PlainText
            }
        };

        Self {
            file_path: input_file.clone(),
            output_format,
        }
    }

    pub const fn file_path(&self) -> &String {
        &self.file_path
    }

    pub const fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}
