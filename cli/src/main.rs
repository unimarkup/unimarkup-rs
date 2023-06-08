use clap::Parser;
use logid::{event_handler::LogEventHandlerBuilder, log, logging::event_entry::AddonKind};
use unimarkup_commons::config::Config;

use crate::log_id::{GeneralError, GeneralInfo};

mod log_id;
mod unimarkup;

fn main() {
    let log_handler = LogEventHandlerBuilder::new()
        .write_to_console()
        .all_log_events()
        .build();

    match Config::try_parse() {
        Ok(cfg) => {
            match unimarkup::compile(cfg) {
                Ok(_) => {
                    log!(GeneralInfo::FinishedCompiling);
                }
                Err(error) => {
                    log!(
                        GeneralError::Compile,
                        add: AddonKind::Info(format!("Cause: {:?}", error))
                    );
                }
            };
        }
        Err(error) => {
            log!(
                GeneralError::ArgParse,
                add: AddonKind::Info(
                    format!("Cause:\n\n{}", error.to_string().replace("error: ", "")),
                )
            );
        }
    }

    log_handler.shutdown();
}
