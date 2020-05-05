use std::env;
mod generator;
use generator::Config;

fn main() {
    let config = Config::new(env::args().collect());
    generator::run(&config);
}
