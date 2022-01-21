use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(Parser)]
#[clap(
    about = "Generate Random password on command line",
    version = "1.1",
    author = "@Sparkenstein"
)]
struct Args {
    #[clap(short, long, help = "Include Uppercase letters")]
    pub upper: bool,

    #[clap(short, long, help = "Include Lowercase letters")]
    pub lower: bool,

    #[clap(short, long, help = "Include Numbers")]
    pub nums: bool,

    #[clap(short, long, help = "Include Symbols")]
    pub symbols: bool,

    #[clap(short = 'L', long, help = "Include Symbols")]
    pub length: usize,
}

fn main() {
    let mut rng = thread_rng();

    let m = Args::parse();

    let mut pass_string = String::from("");
    if m.lower {
        pass_string += "abcdefghijklmnopqrstuvwxyz";
    }

    if m.nums {
        pass_string += "0123456789";
    }

    if m.symbols {
        pass_string += ")(*&^%$#@!~";
    }

    if m.upper {
        pass_string += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    let length = m.length;
    let password = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pass_string.len());
            pass_string.chars().nth(idx).unwrap()
        })
        .collect::<String>();
    println!("{}", password);
}
