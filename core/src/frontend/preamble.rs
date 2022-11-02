//! [`preamble`](crate::frontend::preamble) is the module which implements parsing of the preamble and merge the config of the preamble with the CLI arguments.

use crate::{config::Config, frontend::parser::Rule, log_id::CORE_LOG_ID_MAP};

use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use pest::iterators::Pair;

use super::{log_id::PreambleErrLogId, parser::custom_pest_error};

///[parse_preamble] parses the preamble and tries to serialize the content given either as JSON or YAML into the [Config] struct.
///After serialization, the CLI and preamble config structs are merged with CLI taking precedence.
pub fn parse_preamble(pairs: Pair<Rule>, config: &mut Config) -> Result<(), MappedLogId> {
    let preamble = pairs.into_inner().next().unwrap();

    if preamble.as_rule() == Rule::json_body {
        if let Ok(preamble_config) = serde_json::from_str::<Config>(preamble.as_str()) {
            config.merge(preamble_config);
        } else {
            return Err((PreambleErrLogId::InvalidJSON as LogId).set_event_with(
                &CORE_LOG_ID_MAP,
                &custom_pest_error("Expected valid JSON", preamble.as_span()),
                file!(),
                line!(),
            ));
        }
    }
    if preamble.as_rule() == Rule::yaml_body {
        if let Ok(preamble_config) = serde_yaml::from_str::<Config>(preamble.as_str()) {
            config.merge(preamble_config);
        } else {
            return Err((PreambleErrLogId::InvalidYAML as LogId).set_event_with(
                &CORE_LOG_ID_MAP,
                &custom_pest_error("Expected valid YAML", preamble.as_span()),
                file!(),
                line!(),
            ));
        }
    }
    Ok(())
}
