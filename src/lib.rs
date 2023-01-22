#![allow(dead_code)]

use lexer::location::Source;
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

#[derive(Debug)]
pub struct Compiler {
    config: Config,
    source: Source,
    code: String,
}

impl Compiler {
    pub fn new(config: Config) -> std::io::Result<Self> {
        let mut c = Compiler {
            source: Source::File(config.filename.clone().into()),
            code: String::new(),
            config,
        };
        let mut in_file = std::fs::File::open(&c.config.filename)?;
        in_file.read_to_string(&mut c.code)?;
        Ok(c)
    }
}

pub fn run(config: Config) {
    let mut compiler = Compiler::new(config).expect("TODO: handle errors here");
    let tokens = lexer::tokenize(&mut compiler);

    for t in tokens {
        println!("{t:?}");
    }

    // let ast = parser::parse_tokens(tokens);
    // println!("{ast:#?}");
}
