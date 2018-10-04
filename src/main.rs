#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    let args = App::new("bitwords")
        .version("0.1")
        .about("Generate secure passwords that are easy to type and remember")
        .author("Chris Aumann <me@chr4.org>")
        .arg(
            Arg::with_name("bits")
                .help("Entropy in bits")
                .required(true),
        ).arg(
            Arg::with_name("max_word_length")
                .short("l")
                .long("length")
                .help("Max word length")
                .takes_value(true),
        ).arg(
            Arg::with_name("wordlist")
                .short("f")
                .long("wordlist")
                .help("Specify custom wordlist")
                .takes_value(true),
        ).get_matches();

    let bits = value_t_or_exit!(args.value_of("bits"), u32);
    let max_word_length = value_t!(args.value_of("max_word_length"), usize).unwrap_or(6);
    let filename = args.value_of("wordlist").unwrap_or("wordlist");

    let wordlist = read_wordlist(filename, max_word_length).unwrap();

    println!("Bits: {}", bits);

    for _ in 1..10 {
        let random = rand::thread_rng().choose(&wordlist);
        println!("{:?}", random)
    }
}

// Read in a wordlist, select all words that are longer than max_word_length characters
fn read_wordlist(filename: &str, max_word_length: usize) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
        .filter(|l| l.len() <= max_word_length)
        .collect::<Vec<_>>())
}
