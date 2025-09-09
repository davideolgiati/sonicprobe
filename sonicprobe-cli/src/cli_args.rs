use crate::OutputFormat;

pub struct CliArgs {
    pub(super) file_path: String,
    pub(super) output_format: OutputFormat,
}

impl CliArgs {
    pub const fn file_path(&self) -> &String {
        &self.file_path
    }

    pub const fn output_format(&self) -> &OutputFormat {
        &self.output_format
    }
}
