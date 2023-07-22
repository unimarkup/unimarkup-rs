use clap::Parser;
use logid::{event_handler::builder::LogEventHandlerBuilder, log, logging::event_entry::AddonKind};
use unimarkup_commons::config::{Config, ConfigFns};

use crate::log_id::{GeneralError, GeneralInfo};

mod compiler;
mod log_id;

fn main() {
    let _ = logid::set_filter!("info(infos)");

    let _handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    match Config::try_parse() {
        Ok(cfg) => {
            cfg.validate().unwrap();

            match compiler::compile(cfg) {
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
}
