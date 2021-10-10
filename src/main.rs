extern crate clap;
extern crate colored;
extern crate rand;

use clap::{value_t, App, Arg};
use colored::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::process;

mod wordlist;

fn main() {
    let args = App::new("Correct Horse Battery Staple --- Diceware Passphrase Generator")
        .version("0.3.1")
        .about("Generate secure passphrases that are easy to type and remember")
        .author("Chris Aumann <me@chr4.org>")
        .arg(Arg::with_name("words").help("Number of words in passphrase"))
        .arg(
            Arg::with_name("min_entropy")
                .short("e")
                .long("min-entropy")
                .help("Minimal passphrase entropy [default: 100]")
                .takes_value(true)
                .conflicts_with("words"),
        )
        .arg(
            Arg::with_name("max_word_length")
                .short("l")
                .long("max-word-length")
                .help("Max word length [default: 10]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no_capitalize")
                .short("A")
                .long("no-capitalize")
                .help("Do not capitalize words"),
        )
        .arg(
            Arg::with_name("wordlist")
                .short("f")
                .long("wordlist")
                .help("Specify custom wordlist [default: built-in EFF]")
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
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Do not print entropy information"),
        )
        .get_matches();

    let words = value_t!(args.value_of("words"), usize).unwrap_or(8);
    let min_entropy = value_t!(args.value_of("min_entropy"), f64).unwrap_or(100.0);
    let max_word_length = value_t!(args.value_of("max_word_length"), usize).unwrap_or(6);
    let append_number = !args.is_present("no_append_number");
    let capitalize = !args.is_present("no_capitalize");
    let delimiter = args.value_of("delimiter").unwrap_or("-");
    let quiet = args.is_present("quiet");

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

    let mut pwd: Vec<String> = vec![];
    let mut rng = thread_rng();
    let mut entropy: f64;
    let wordlist_len = if args.is_present("wordlist") {
        wordlist_file.len()
    } else {
        wordlist::WORDLIST.len()
    };

    // Choose random words from the wordlist and append them to the passphrase
    loop {
        let word_str = if args.is_present("wordlist") {
            match wordlist_file.choose(&mut rng) {
                Some(s) => s,
                None => continue,
            }
        } else {
            match wordlist::WORDLIST.choose(&mut rng) {
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

        // Capitalize word unless --no-capitalize was set and it add to list
        pwd.push(if capitalize {
            word_str.to_string().to_capitalized()
        } else {
            word_str.to_string()
        });

        entropy = calculate_entropy(wordlist_len, pwd.len());

        // Keep going until requirements are met
        if args.is_present("words") {
            if pwd.len() >= words {
                break;
            }
        } else {
            if entropy >= min_entropy {
                break;
            }
        }
    }

    // Insert a random number from 0-9 at a random location if --add-number was specified
    if append_number {
        let no = thread_rng().gen_range(0..10).to_string();
        let i = thread_rng().gen_range(0..pwd.len());
        pwd.insert(i, no);

        // Update entropy, numeric value has 10 n
        entropy += (10 as f64).log(2.0)
    }

    // Concatinate words with dashes and print the passphrase
    println!("{}", pwd.join(delimiter));

    // Print entropy to stderr and evaluate passphrase strength
    if !quiet {
        let entropy_str = format!("Entropy: {:0.2} bits", entropy);

        if entropy < 70.0 {
            eprintln!("😫 {} {}", entropy_str.bold(), "(not secure)".red().bold());
        } else if entropy < 95.0 {
            eprintln!("🤨 {} {}", entropy_str.bold(), "(decent)".yellow().bold());
        } else if entropy < 120.0 {
            eprintln!("😊 {} {}", entropy_str.bold(), "(good)".green().bold());
        } else {
            eprintln!("😎 {} {}", entropy_str.bold(), "(paranoid)".blue().bold());
        }
    }
}

fn calculate_entropy(wordlist_length: usize, word_count: usize) -> f64 {
    (wordlist_length as f64).log(2.0) * word_count as f64
}

#[test]
fn calculate_entropy_test() {
    assert_eq!(calculate_entropy(7776, 6), 77.54887502163469);
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
