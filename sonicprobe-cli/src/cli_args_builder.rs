use sonicprobe_core::sonicprobe_error::SonicProbeError;

use crate::{
    cli_args::CliArgs, OutputFormat
    
};

pub fn cli_args_from_args_array(args: &[String]) -> Result<CliArgs, SonicProbeError> {
    assert!((args.len() >= 2), "No input file specified");

    let input_file = args.get(1).map_or_else(
        || {
            Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: "no input file provided".to_owned(),
            })
        },
        Ok,
    )?;

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

    Ok(CliArgs {
        file_path: input_file.clone(),
        output_format,
    })
}
