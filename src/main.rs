use clap::Parser;
use simple_logger::SimpleLogger;
use unimarkup_rs::{config::Config, unimarkup};

fn main() {
    SimpleLogger::new().init().unwrap();
    let config = Config::parse();

    match unimarkup::compile(config) {
        Ok(_) => println!("Done"),
        Err(err) => println!("Error: {}", err),
    }
}
