#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let args = App::new("bitwords")
        .version("0.1")
        .about("Generate secure passwords that are easy to type and remember")
        .author("Chris Aumann <me@chr4.org>")
        .arg(
            Arg::with_name("bits")
                .help("How many bits do you want")
                .required(true),
        ).get_matches();

    let bits = value_t_or_exit!(args.value_of("bits"), u32);
    println!("Bits: {}", bits);
}
