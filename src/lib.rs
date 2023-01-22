#![allow(dead_code)]

use std::env;
use std::io::Read;

mod lexer;
// mod parser;
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

pub fn run(config: Config) {
    let mut file = std::fs::File::open(config.filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let tokens = lexer::tokenize(&buf);

    for t in tokens {
        println!("{t:?}");
    }

    // let ast = parser::parse_tokens(tokens);
    // println!("{ast:#?}");
}
