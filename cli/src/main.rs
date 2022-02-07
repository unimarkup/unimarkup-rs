use clap::Parser;
use log::{error, info};
use unimarkup_core::config::Config;

mod logger;
mod unimarkup;

fn main() {
    logger::init_logger();

    match Config::try_parse() {
        Ok(config) => {
            match unimarkup::compile(config) {
                Ok(_) => info!("Done"),
                Err(err) => error!("Error: {}", err),
            };
        }
        Err(error) => {
            let msg = error.to_string().replace("error: ", "");

            error!("{}", msg);
        }
    }
}
