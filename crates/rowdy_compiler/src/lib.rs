use rowdy_location::Source;
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
    pub config: Config,
    pub code: String,
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
