use clap::Parser;
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(long, default_value = "12")]
    pub length: usize,

    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}
