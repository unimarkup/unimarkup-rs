use clap::Parser;
use log::{error, info};
use unimarkup_rs::logger;
use unimarkup_rs::{config::Config, unimarkup};

fn main() {
    logger::init_logger();

    let config = Config::parse();

    match unimarkup::compile(config) {
        Ok(_) => info!("Done"),
        Err(err) => error!("Error: {}", err),
    }
}
