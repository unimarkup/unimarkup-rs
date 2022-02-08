use clap::Parser;
use log_id::GeneralInfLogId;
use unimarkup_core::{config::Config, log_id::{SetLog, LogId}};

use crate::log_id::GeneralErrLogId;

mod logger;
mod unimarkup;
mod error;
mod log_id;

fn main() {
    logger::init_logger();

    match Config::try_parse() {
        Ok(config) => {
            let um_file = config.um_file.clone();

            match unimarkup::compile(config) {
                Ok(_) => {
                    (GeneralInfLogId::FinishedCompiling as LogId)
                    .set_log(&format!("Finished compiling: {:?}", um_file), file!(), line!());
                },
                Err(err) => {
                    (GeneralErrLogId::FailedCompiling as LogId)
                    .set_log(&format!("Failed compiling: {:?}", um_file), file!(), line!())
                    .add_info(&format!("Cause: {:?}", err));
                },
            };
        }
        Err(error) => {
            (GeneralErrLogId::FailedParsingArgs as LogId)
            .set_log("Failed parsing comandline arguments!", file!(), line!())
            .add_info(&format!("Cause: {}", error.to_string().replace("error: ", "")));
        }
    }
}
