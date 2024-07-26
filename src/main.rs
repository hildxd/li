use anyhow::Result;
use clap::Parser;
use li::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand, TextSubCommand,
};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(opts),
        SubCommand::GenPass(opts) => process_genpass(&opts),
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format),
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format),
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => todo!(),
            TextSubCommand::Verify(opts) => todo!(),
        },
    }
}
