#[macro_use]
extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::process;

fn main() {
    let args = App::new("bitwords")
        .version("0.1")
        .about("Generate secure passphrases that are easy to type and remember")
        .author("Chris Aumann <me@chr4.org>")
        .arg(Arg::with_name("passphrase_length").help("passphrase length"))
        .arg(
            Arg::with_name("max_word_length")
                .short("l")
                .long("length")
                .help("Max word length")
                .takes_value(true),
        ).arg(
            Arg::with_name("capitalize")
                .short("A")
                .long("no-capitalize")
                .help("Do not capitalize words")
                .takes_value(true),
        ).arg(
            Arg::with_name("wordlist")
                .short("f")
                .long("wordlist")
                .help("Specify custom wordlist")
                .takes_value(true),
        ).get_matches();

    let passphrase_length = value_t!(args.value_of("passphrase_length"), usize).unwrap_or(24);
    let max_word_length = value_t!(args.value_of("max_word_length"), usize).unwrap_or(6);
    let capitalize = value_t!(args.value_of("capitalize"), bool).unwrap_or(true);
    let wordlist_filename = args.value_of("wordlist").unwrap_or("wordlist");

    let wordlist = match read_wordlist(wordlist_filename, max_word_length) {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error reading in wordlist: {}", e);
            process::exit(1);
        }
    };

    // Choose random words from the wordlist and append them to the passphrase until length is met
    let mut pwd: Vec<String> = vec![];
    while pwd.join("-").len() < passphrase_length {
        let word_str = match rand::thread_rng().choose(&wordlist) {
            Some(s) => s,
            None => continue,
        };

        // Capitalize word if capitalize flag was set and add to list
        pwd.push(if capitalize {
            word_str.to_string().to_capitalized()
        } else {
            word_str.to_string()
        });
    }

    // Concatinate words with dashes and print the passphrase!
    println!("{}", pwd.join("-"))
}

// Read in a wordlist, select all words that are longer than max_word_length characters.
// Lines starting with a # will be skipped.
fn read_wordlist(filename: &str, max_word_length: usize) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())       // Skip erroneous lines
        .filter(|l| !l.starts_with("#"))        // Skip comments
        .filter(|l| l.len() <= max_word_length) // Filter out too long words
        .collect::<Vec<_>>())
}

// This was taken from https://stackoverflow.com/a/38343355
trait ToCapitalized {
    fn to_capitalized(&self) -> String;
}

impl ToCapitalized for String {
    fn to_capitalized(&self) -> String {
        let mut s = String::with_capacity(self.len());
        let mut chars = self.chars();

        s.extend(chars.by_ref().take(1).flat_map(|c| c.to_uppercase()));
        s.extend(chars.flat_map(|c| c.to_lowercase()));

        s
    }
}
