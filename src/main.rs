extern crate clap;
extern crate rand;

use clap::{value_t, App, Arg};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::process;

mod wordlist;

fn main() {
    let args = App::new("Correct Horse Battery Staple --- Password Generator")
        .version("0.1")
        .about("Generate secure passphrases that are easy to type and remember")
        .author("Chris Aumann <me@chr4.org>")
        .arg(Arg::with_name("min_passphrase_length").help("Min passphrase length [default: 24]"))
        .arg(
            Arg::with_name("max_word_length")
                .short("l")
                .long("max-word-length")
                .help("Max word length [default: 6]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no_capitalize")
                .short("A")
                .long("no-capitalize")
                .help("Do not capitalize words [default: true]"),
        )
        .arg(
            Arg::with_name("wordlist")
                .short("f")
                .long("wordlist")
                .help("Specify custom wordlist [default: built-in]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .help("Use custom delimiter [default: '-']")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no_append_number")
                .short("n")
                .long("no-append-number")
                .help("Do not append a random number at the end"),
        )
        .get_matches();

    let min_passphrase_length =
        value_t!(args.value_of("min_passphrase_length"), usize).unwrap_or(24);
    let min_words = min_passphrase_length / 5;
    let max_word_length = value_t!(args.value_of("max_word_length"), usize).unwrap_or(6);
    let append_number = !args.is_present("no_append_number");
    let capitalize = !args.is_present("no_capitalize");
    let delimiter = args.value_of("delimiter").unwrap_or("-");

    // If a wordlist is specified, read it in
    let mut wordlist_file: Vec<String> = vec![];
    if args.is_present("wordlist") {
        let wordlist_filename = args.value_of("wordlist").unwrap_or("");
        wordlist_file = match read_wordlist(wordlist_filename, max_word_length) {
            Ok(list) => list,
            Err(e) => {
                eprintln!("Error reading in wordlist: {}", e);
                process::exit(1);
            }
        };
    }

    // Choose random words from the wordlist and append them to the passphrase until length is met
    let mut pwd: Vec<String> = vec![];
    while pwd.len() < min_words || pwd.join(delimiter).len() < min_passphrase_length {
        let word_str = if args.is_present("wordlist") {
            match rand::thread_rng().choose(&wordlist_file) {
                Some(s) => s,
                None => continue,
            }
        } else {
            match rand::thread_rng().choose(&wordlist::WORDLIST) {
                Some(s) => {
                    // Filter out too long words
                    if s.len() > max_word_length {
                        continue;
                    }
                    *s
                }
                None => continue,
            }
        };

        // Capitalize word if --capitalize was set and it add to list
        pwd.push(if capitalize {
            word_str.to_string().to_capitalized()
        } else {
            word_str.to_string()
        });
    }

    // Append a random number from 0-9 if --add-number was specified
    if append_number {
        pwd.push(rand::thread_rng().gen_range(0, 10).to_string());
    }

    // Concatinate words with dashes and print the passphrase!
    println!("{}", pwd.join(delimiter))
}

// Read in a wordlist, select all words that are longer than max_word_length characters.
// Lines starting with a # will be skipped.
fn read_wordlist(filename: &str, max_word_length: usize) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok()) // Skip erroneous lines
        .filter(|l| !l.starts_with("#")) // Skip comments
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
