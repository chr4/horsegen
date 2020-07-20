# Horsegen - Secure passphrases that are easy to type and remember

Horsegen is a console password generator using the famous diceware "correct horse battery staple" schema (hence the name).

It is written in [Rust](https://rust-lang.org) and runs on Linux and macOS.

Uses the (built-in) EFF diceword wordlist by default, but can be used with other wordlists as well.

## Usage

```
Correct Horse Battery Staple --- Password Generator 0.3
Chris Aumann <me@chr4.org>
Generate secure passphrases that are easy to type and remember

USAGE:
    horsegen [FLAGS] [OPTIONS] [words]

FLAGS:
    -h, --help                Prints help information
    -n, --no-append-number    Do not append a random number at the end
    -A, --no-capitalize       Do not capitalize words [default: true]
    -q, --quiet               Do not print entropy information
    -V, --version             Prints version information

OPTIONS:
    -d, --delimiter <delimiter>                Use custom delimiter [default: '-']
    -l, --max-word-length <max_word_length>    Max word length [default: 10]
    -e, --min-entropy <min_entropy>            Minimal password entropy [default: 100]
    -f, --wordlist <wordlist>                  Specify custom wordlist [default: built-in EFF]

ARGS:
    <words>    Number of words in passphrase
```


## Similar tools

- [diceware](https://github.com/ulif/diceware), written in Python
- [pwgen](https://pwgen.sourceforge.io)
