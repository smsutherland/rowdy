use std::env;
use std::process;

use rowdy::Config;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(0);
        }
    };

    rowdy::run(config);
}
