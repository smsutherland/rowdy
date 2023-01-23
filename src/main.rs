use rowdy::Config;
use std::process;

fn main() {
    let config = match Config::new_from_args() {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(0);
        }
    };

    rowdy::run(config);
}
