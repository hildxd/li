use clap::Parser;

mod csv;
mod genpass;

pub use self::{csv::*, genpass::*};

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
}
