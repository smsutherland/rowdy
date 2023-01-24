#![allow(dead_code)]

mod lexer;
mod location;
mod parser;
mod types;

use location::Source;
use std::io::Read;

#[derive(Debug)]
pub struct Config {
    pub source: Source,
}

impl Config {
    pub fn new_from_args() -> Result<Config, &'static str> {
        let mut args = std::env::args_os();
        args.next(); // program name
        let source = match args.next() {
            Some(val) => Source::File(val),
            None => return Err("Didn't get a filename"),
        };

        Ok(Config { source })
    }
}

#[derive(Debug)]
pub struct Compiler {
    config: Config,
    code: String,
}

impl Compiler {
    pub fn new(config: Config) -> std::io::Result<Self> {
        let mut c = Compiler {
            code: String::new(),
            config,
        };
        match &c.config.source {
            Source::File(fname) => {
                let mut in_file = std::fs::File::open(fname)?;
                in_file.read_to_string(&mut c.code)?;
            }
            Source::Anonymous => {}
        }
        Ok(c)
    }
}

pub fn run(config: Config) {
    let compiler = Compiler::new(config).expect("TODO: handle errors here");
    let tokens = lexer::tokenize(&compiler);

    for t in tokens.clone() {
        println!("{t:?}");
    }

    let ast = parser::parse_tokens(tokens);
    println!("{ast:#?}");
}

#[test]
fn compile_testry() {
    let config = Config {
        source: Source::File("./test.ry".into())
    };
    run(config);
}
