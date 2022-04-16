#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod lexer;
mod parser;
mod types;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a filename"),
        };

        Ok(Config { filename })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run(config: Config) {
    let tokens = lexer::lex_file(&config.filename).unwrap();

    // for t in tokens {
    //     println!("{t:?}");
    // }

    let ast = parser::parse_tokens(tokens);
    println!("{ast:#?}");
}
