//! # aɪ pʰiː eɪ
//! *aɪ pʰiː eɪ* is an esoteric, two-dimensional, programming language,
//! where all commands are represented by
//! [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet) symbols.
//!
//! Rather than using a stack, it uses a grid like <i forget what its called>.
//!
//! It also supports befunge-style arrows! Yay!

use clap::Parser;
use std::path::PathBuf;

/// The [clap](https://crates.io/crates/clap) CLI parser.
#[derive(Parser, Debug)]
#[command(
    name = "aɪ pʰiː eɪ",
    version = "0.1.0",
    author = "William Nelson",
    about = "An esoteric programming language, where all commands are represented by IPA symbols.",
    long_about = "aɪ pʰiː eɪ is an esoteric, two-dimensional, programming language, where all \
     commands are represented by IPA symbols. Rather than using a stack, it uses a grid like \
     <i forget what its called>. It also supports befunge-style arrows!"
)]
struct Cli {
    /// The file to run.
    file: PathBuf,
}

/// The entry point
fn main() {
    let args: Cli = Cli::parse();
    if !(args.file.exists() && args.file.is_file()) {
        panic!("File does not exist!");
    } else if args.file.extension().unwrap() != "ipa" {
        panic!("File is not a aɪpʰiːeɪ file!");
    } else {
        println!("Running file: {}", args.file.display());
        let file_contents: Vec<Vec<char>> = std::fs::read_to_string(args.file)
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        for l in file_contents {
            for c in l {
                if c == '\t' {
                    panic!("Tabs are not allowed! Get outta here, ya filthy animal!");
                }
                print!("{}", c);
            }
            println!();
        }
    }
}
