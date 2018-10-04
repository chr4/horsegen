# Horsegen - Secure passphrases that are easy to type and remember

Horsegen is a console password generator using the famous "correct horse
battert staple" schema (hence the name).

It is written in [Rust](https://rust-lang.org) and runs on Linux and macOS.

Words in built-in database: `80549`

## Usage

```
Correct Horse Battery Staple --- Password Generator 0.1
Chris Aumann <me@chr4.org>
Generate secure passphrases that are easy to type and remember

USAGE:
    horsegen [FLAGS] [OPTIONS] [min_passphrase_length]

FLAGS:
    -h, --help                Prints help information
    -n, --no-append-number    Do not append a random number at the end
    -A, --no-capitalize       Do not capitalize words [default: true]
    -V, --version             Prints version information

OPTIONS:
    -l, --max-word-length <max_word_length>    Max word length [default: 6]
    -w, --min-words <min_words>                Min number of words [default: 4]
    -s, --seperator <seperator>                Use custom seperator [default: '-']
    -f, --wordlist <wordlist>                  Specify custom wordlist [default: built-in]

ARGS:
    <min_passphrase_length>    Min passphrase length [default: 24]
```
