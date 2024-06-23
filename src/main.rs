use clap::Parser;
use template::{process_csv, Opts, SubCommand};

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(opts),
    };
}
