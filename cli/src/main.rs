use clap::Parser;
use logid::{
    event_handler::builder::LogEventHandlerBuilder,
    log,
    log_id::LogLevel,
    logging::{
        event_entry::AddonKind,
        filter::{AddonFilter, FilterConfigBuilder},
    },
};
use unimarkup_core::commons::config::Config;

use crate::log_id::{GeneralError, GeneralInfo};

mod compiler;
mod log_id;

fn main() {
    let _ = logid::logging::filter::set_filter(
        FilterConfigBuilder::new(LogLevel::Info)
            .allowed_addons(AddonFilter::Infos)
            .build(),
    );

    let _handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build();

    let config = Config::parse();
    match compiler::compile(config) {
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
