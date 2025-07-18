use crate::OutputFormat;

pub struct CliArgs {
        file_path: String,
        output_format: OutputFormat
}

impl CliArgs {
        pub fn new(args: &[String]) -> CliArgs {
                if args.len() < 2 {
                        panic!("No input file specified")
                }
        
                let input_file = &args[1];
                let output_format: OutputFormat = {
                        if args.len() >= 3 && args.iter().any(|option| option.eq_ignore_ascii_case("--json")) {
                                OutputFormat::Json
                        } else {
                                OutputFormat::PlainText
                        }
                };
                
                CliArgs {
                        file_path: input_file.clone(),
                        output_format
                }
        }

        pub fn file_path(&self) -> &String {
                &self.file_path
        }

        pub fn output_format(&self) -> &OutputFormat {
                &self.output_format
        }
}