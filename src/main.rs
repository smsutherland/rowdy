use std::env;
use rowdy::Config;
use std::process;

fn main() {
    let config = match Config::new(env::args()){
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(0);
        }
    };

    let lines = match rowdy::read_lines(&config.filename){
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(0);
        }
    };

    for line in lines{
        if let Ok(line) = line{
            println!("{}", line);
        }
    }
}
