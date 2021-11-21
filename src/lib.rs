use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

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

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}