use std::path::Path;

use clap::Parser;

mod base64;
mod csv;
mod genpass;
mod text;

pub use self::{base64::*, csv::*, genpass::*, text::*};

#[derive(Debug, Parser)]
#[command(name ="li", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats.")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password.")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}
