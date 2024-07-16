use core::fmt;
use std::{fmt::Display, str::FromStr};

use clap::Parser;

use crate::verify_input_file;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,

    #[arg(long, short, default_value = "json")]
    pub format: Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Json,
    Yaml,
}

impl From<Format> for &'static str {
    fn from(format: Format) -> Self {
        match format {
            Format::Json => "json",
            Format::Yaml => "yaml",
        }
    }
}

impl FromStr for Format {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            _ => Err("Invalid format"),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
