use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

mod lexer;

#[derive(Debug)]
pub struct Config{
    pub filename: String,
}

impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();
        let filename = match args.next(){
            Some(val) => val,
            None => return Err("Didn't get a filename"),
        };
    
        Ok(Config{ filename })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run(config: Config){

    let tokens = lexer::lex_file(&config.filename).unwrap();

    for token in &tokens{
        println!("{:?}", token);
    }
    println!("{}", tokens.len());
}