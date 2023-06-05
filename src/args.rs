use clap::Parser;

#[derive(Parser)]
#[clap(
    about = "Generate Random password on command line",
    version = "1.1",
    author = "@Sparkenstein"
)]
pub struct Args {
    #[clap(short, long, help = "Include Uppercase letters")]
    pub upper: bool,

    #[clap(short, long, help = "Include Lowercase letters")]
    pub lower: bool,

    #[clap(short, long, help = "Include Numbers")]
    pub nums: bool,

    #[clap(short, long, help = "Include Symbols")]
    pub symbols: bool,

    #[clap(short = 'L', long, help = "Set password length")]
    pub length: usize,

    #[clap(short, long, help = "Set password count")]
    pub passwords: usize,

    #[clap(short, long, help = "Calculate entropy")]
    pub entropy: bool,
}