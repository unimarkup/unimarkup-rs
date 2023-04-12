use clap::Parser;
use log_id::CLI_LOG_ID_MAP;
use logid::{capturing::LogIdTracing, log_id::LogId};
use unimarkup_core::config::Config;

use crate::log_id::{GeneralErrLogId, GeneralInfLogId};

mod log_id;
mod logger;
mod unimarkup;

fn main() {
    logger::init_logger();

    match Config::try_parse() {
        Ok(config) => {
            let um_file = config.um_file.clone();

            match unimarkup::compile(config) {
                Ok(_) => {
                    (GeneralInfLogId::FinishedCompiling as LogId).set_event_with(
                        &CLI_LOG_ID_MAP,
                        &format!("Finished compiling: {:?}", um_file),
                        file!(),
                        line!(),
                    );
                }
                Err(err) => {
                    (GeneralErrLogId::FailedCompiling as LogId)
                        .set_event_with(
                            &CLI_LOG_ID_MAP,
                            &format!("Failed compiling: {:?}", um_file),
                            file!(),
                            line!(),
                        )
                        .add_info(&format!("Cause: {:?}", err));
                }
            };
        }
        Err(error) => {
            (GeneralErrLogId::FailedParsingArgs as LogId)
                .set_event_with(
                    &CLI_LOG_ID_MAP,
                    "Failed parsing comandline arguments!",
                    file!(),
                    line!(),
                )
                .add_info(&format!(
                    "Cause: {}",
                    error.to_string().replace("error: ", "")
                ));
        }
    }
}
