extern crate clap;

use clap::{App, Arg};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let matches = App::new("passgen")
        .version("1.0")
        .author("@Sparkenstein")
        .about("Generate Random password on command line")
        .arg(
            Arg::with_name("include")
                .long("include")
                .short("i")
                .default_value("nlus")
                .help("include numbers/upppercase/lowercase/symbols in generated string"),
        )
        .arg(
            Arg::with_name("length")
                .long("length")
                .short("l")
                .takes_value(true)
                .default_value("16")
                .help("include lowercase characters in generated string"),
        )
        .get_matches();
    let chars_to_include = matches.value_of("include").unwrap();
    let mut pass_string = String::from("");
    if chars_to_include.contains("l") {
        pass_string += "abcdefghijklmnopqrstuvwxyz";
    }

    if chars_to_include.contains("n") {
        pass_string += "0123456789";
    }

    if chars_to_include.contains("s") {
        pass_string += ")(*&^%$#@!~";
    }

    if chars_to_include.contains("u") {
        pass_string += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    let length = matches
        .value_of("length")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let password = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pass_string.len());
            pass_string.chars().nth(idx).unwrap()
        })
        .collect::<String>();
    println!("{}", password);
}
