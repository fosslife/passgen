mod args;

use clap::Parser;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let m = args::Args::parse();

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

    for password_number in 1..m.passwords {
        let password = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..pass_string.len());
                pass_string.chars().nth(idx).unwrap()
            })
            .collect::<String>();

        if m.entropy {
            let e = calculate_entropy(length, pass_string.len());
            // ref: https://iocane.com.au/talking-passwords-and-entropy/
            let quality = match e.round() as usize {
                d if d < 28 => "Very Weak",
                28..=35 => "Weak",
                36..=59 => "Reasonable",
                60..=127 => "Strong",
                d if d > 128 => "Very Strong",
                _ => "Unknown entropy detected, this should never happen",
            };
            println!("{}. {} | Entropy: {:.3}bits [{}]", password_number, password, e, quality);
        } else {
            println!("{}. {}", password_number, password);
        }
    }
}

fn calculate_entropy(l: usize, c: usize) -> f64 {
    let chars = c as f64;
    let pow = chars.powf(l as f64);
    return pow.log2();
}
