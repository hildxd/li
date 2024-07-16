use anyhow::Result;
use clap::Parser;
use li::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(opts),
        SubCommand::GenPass(opts) => process_genpass(&opts),
    }?;
    Ok(())
}
