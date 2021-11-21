use std::env;

#[derive(Debug)]
pub struct Config{
    filename: String,
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
