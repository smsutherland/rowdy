use std::env;
use rowdy::Config;

fn main() {
    let config = Config::new(env::args());
    println!("{:?}", config);
}
