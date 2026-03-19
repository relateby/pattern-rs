use crate::cli::OutputFormatArg;
use std::io::{self, IsTerminal};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    Gram,
    Text,
    Json,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(value: OutputFormatArg) -> Self {
        match value {
            OutputFormatArg::Gram => OutputFormat::Gram,
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutputContext {
    pub format: OutputFormat,
    pub use_color: bool,
}

impl OutputContext {
    pub fn new(format: OutputFormat) -> Self {
        Self {
            format,
            use_color: io::stdout().is_terminal(),
        }
    }
}
