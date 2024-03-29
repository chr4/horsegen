extern crate clap;
extern crate owo_colors;
extern crate rand;

use clap::{value_parser, Arg, ArgAction, Command};
use owo_colors::OwoColorize;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::process;

// Update entropy, numeric value has (10 as f64).log(2.0)
const ENTROPY_NUMERIC_VALUE: f64 = 3.3219280948873626;

fn main() {
    let args = Command::new("Correct Horse Battery Staple --- Diceware Passphrase Generator")
        .version("0.3.3")
        .about("Generate secure passphrases that are easy to type and remember")
        .author("Chris Aumann <me@chr4.org>")
        .arg(
            Arg::new("words")
                .help("Number of words in passphrase")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("min_entropy")
                .short('e')
                .long("min-entropy")
                .help("Minimal passphrase entropy [default: 100]")
                .value_parser(value_parser!(f64))
                .default_value("100.0")
                .conflicts_with("words"),
        )
        .arg(
            Arg::new("max_word_length")
                .short('l')
                .long("max-word-length")
                .value_parser(value_parser!(usize))
                .default_value("10")
                .help("Max word length"),
        )
        .arg(
            Arg::new("no_capitalize")
                .short('A')
                .long("no-capitalize")
                .action(ArgAction::SetFalse)
                .help("Do not capitalize words"),
        )
        .arg(
            Arg::new("wordlist")
                .short('f')
                .long("wordlist")
                .help("Specify custom wordlist [default: built-in EFF]"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .default_value("-")
                .help("Use custom delimiter"),
        )
        .arg(
            Arg::new("no_append_number")
                .short('n')
                .long("no-append-number")
                .action(ArgAction::SetFalse)
                .help("Do not append a random number at the end"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Do not print entropy information"),
        )
        .get_matches();

    let words = args.get_one("words");
    let min_entropy = args.get_one("min_entropy").unwrap();
    let max_word_length = args.get_one("max_word_length").unwrap();
    let append_number = args.get_flag("no_append_number");
    let capitalize = args.get_flag("no_capitalize");
    let delimiter = args.get_one::<String>("delimiter").unwrap();
    let quiet = args.get_flag("quiet");

    let wordlist_file = match args.get_one::<String>("wordlist") {
        Some(filename) => match std::fs::read_to_string(filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error reading in wordlist: {}", e);
                process::exit(1);
            }
        },
        None => include_str!("../eff_large_wordlist.txt").to_string(),
    };

    // Read in a wordlist, select all words that are longer than max_word_length characters.
    // Lines starting with a # will be skipped.
    let wordlist = wordlist_file
        .lines()
        .filter(|l| !l.starts_with('#')) // Skip comments
        .filter(|l| l.len() <= *max_word_length) // Filter out too long words
        .collect::<Vec<_>>();

    let mut pwd: Vec<String> = vec![];
    let mut rng = thread_rng();
    let mut entropy: f64;

    // Choose random words from the wordlist and append them to the passphrase
    loop {
        let word_str = match wordlist.choose(&mut rng) {
            Some(s) => s,
            None => continue,
        };

        // Capitalize word unless --no-capitalize was set and it add to list
        pwd.push(if capitalize {
            word_str.to_string().to_capitalized()
        } else {
            word_str.to_string()
        });

        entropy = calculate_entropy(wordlist.len(), pwd.len());

        // Keep going until requirements are met
        match words {
            Some(w) => {
                if pwd.len() >= *w {
                    break;
                }
            }
            None => {
                // If append_number is set, factor in additional entropy, even though the number is
                // only added afterwards
                let required_entropy = if append_number {
                    *min_entropy - ENTROPY_NUMERIC_VALUE
                } else {
                    *min_entropy
                };

                if entropy >= required_entropy {
                    break;
                }
            }
        }
    }

    // Insert a random number from 0-9 at a random location if --add-number was specified
    if append_number {
        let no = thread_rng().gen_range(0..10).to_string();
        let i = thread_rng().gen_range(0..pwd.len());
        pwd.insert(i, no);

        entropy += ENTROPY_NUMERIC_VALUE;
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
