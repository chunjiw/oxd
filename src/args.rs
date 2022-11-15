use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct OxdArgs {
    /// The term to look up
    pub word: String,

    /// Play pronunciation file from API
    #[arg(short, long, default_value_t = false)]
    pub sound: bool,
}
