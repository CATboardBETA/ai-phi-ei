//! # aɪ pʰiː eɪ
//! *aɪ pʰiː eɪ* is an esoteric, two-dimensional, programming language,
//! where all commands are represented by
//! [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet) symbols.
//!
//! It also supports befunge-style arrows! Yay!

pub mod cursor;

use clap::{arg, value_parser, Arg, Command};
use std::intrinsics::unreachable;
use std::path::{Path, PathBuf};
use crate::cursor::Cursor2d;

struct Args {
    file: PathBuf,
    verbose: bool,
    quiet: bool,
    debug: bool,
}

fn main() {
    let arg_matches = Command::new("ai-phi-eɪ")
        .version("0.1.0")
        .author("William Nelson aka CATboardBETA <catboardbeta@gmail.com>")
        .about("A two-dimensional esoteric programming language")
        .arg(
            Arg::with_name("input")
                .help("The file to run")
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .index(1),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Prints information about the program as it runs. May be used up to 3 times")
                .short("v")
                .long("verbose")
                .long("verb")
                .multiple(true)
                .takes_value(false)
                .required(false)
                .conflicts_with("quiet"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Does not print information about the program as it runs")
                .short("q")
                .long("quiet")
                .multiple(false)
                .takes_value(false)
                .required(false)
                .conflicts_with("verbose"),
        )
        .arg(
            Arg::with_name("debug")
                .help("Prints debug information about the program as it runs")
                .short("d")
                .long("debug")
                .multiple(false)
                .takes_value(false)
                .required(false)
                .conflicts_with("quiet"),
        );

    let arg_matches = arg_matches.get_matches();
    let args = Args {
        file: arg_matches
            .get_one::<PathBuf>("input")
            .expect("file is required")
            .take_mut(),
        verbose: *arg_matches.get_one::<bool>("verbose").unwrap_or(&false),
        quiet: *arg_matches.get_one::<bool>("quiet").unwrap_or(&false),
        debug: *arg_matches.get_one::<bool>("debug").unwrap_or(&false),
    };

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
                    panic!("Tabs are not allowed!");
                }
            }
            println!();
        }

        let mut cursor = Cursor2d::new(file_contents);
        let mut stack: Vec<i32> = Vec::new();
        let mut output = String::new();
        let mut input = String::new();

        while let Some(c) = cursor.next() {
            match c {
                'θ' => {
                    if let Some(x) = cursor.get_next() {
                        if x == "#" {
                            while let Some(x) = cursor.get_next() {
                                if x == ";" {
                                    break;
                                } else {
                                    stack.push(x.parse::<i32>().expect(&format!("Expected integer after #: {}, {}", cursor.x, cursor.y)));
                                }
                            }
                        } else if x == "&" {
                            cursor.move_
                            while let Some(x) = cursor.get_next() {
                                if x == ";" {
                                    break;
                                } else {
                                    input.push_str(&x);
                                }
                            }
                        } else if x == ";" {
                            panic!(&format!("Expected # or & after θ: {}, {}", cursor.x, cursor.y));
                        } else if x == ""
                    }
                }
                _ => {
                    panic!("Unknown command: {}", c);
                }
            }
        }
    }
}
