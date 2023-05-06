use clap::Parser;
use log_id::CLI_LOG_ID_MAP;
use logid::{capturing::LogIdTracing, log_id::LogId};
use unimarkup_commons::config::Config;

use crate::log_id::{GeneralErrLogId, GeneralInfLogId};

mod log_id;
mod logger;
mod unimarkup;

fn main() {
    logger::init_logger();

    match Config::try_parse() {
        Ok(cfg) => {
            let input = cfg.input.clone();

            match unimarkup::compile(cfg) {
                Ok(_) => {
                    (GeneralInfLogId::FinishedCompiling as LogId).set_event_with(
                        &CLI_LOG_ID_MAP,
                        &format!("Finished compiling: {:?}", input),
                        file!(),
                        line!(),
                    );
                }
                Err(err) => {
                    (GeneralErrLogId::FailedCompiling as LogId)
                        .set_event_with(
                            &CLI_LOG_ID_MAP,
                            &format!("Failed compiling: {:?}", input),
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
                    "Cause:\n\n{}",
                    error.to_string().replace("error: ", "")
                ));
        }
    }
}
