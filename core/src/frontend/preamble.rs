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

impl Config {
    /// Merges the fields of two [`Config`]s.
    /// Any field that is `None` is taken from `other` [`Config`] if available.
    ///
    /// In other words, the fields of [`Config`] that this method is called on, take precedence over the
    /// fields of the `other` [`Config`].
    pub fn merge(&mut self, other: Config) {
        if self.out_file.is_none() && other.out_file.is_some() {
            self.out_file = other.out_file;
        }
        if self.out_formats.is_none() && other.out_formats.is_some() {
            self.out_formats = other.out_formats;
        }
        if self.insert_paths.is_none() && other.insert_paths.is_some() {
            self.insert_paths = other.insert_paths;
        }
        if self.dot_unimarkup.is_none() && other.dot_unimarkup.is_some() {
            self.dot_unimarkup = other.dot_unimarkup;
        }
        if self.theme.is_none() && other.theme.is_some() {
            self.theme = other.theme;
        }
        if self.flags.is_none() && other.flags.is_some() {
            self.flags = other.flags;
        }
        if self.enable_elements.is_none() && other.enable_elements.is_some() {
            self.enable_elements = other.enable_elements;
        }
        if self.disable_elements.is_none() && other.disable_elements.is_some() {
            self.disable_elements = other.disable_elements;
        }
        if self.citation_style.is_none() && other.citation_style.is_some() {
            self.citation_style = other.citation_style;
        }
        if self.references.is_none() && other.references.is_some() {
            self.references = other.references;
        }
        if self.fonts.is_none() && other.fonts.is_some() {
            self.fonts = other.fonts;
        }
        if !self.overwrite_out_files && other.overwrite_out_files {
            self.overwrite_out_files = other.overwrite_out_files;
        }
        if !self.clean && other.clean {
            self.clean = other.clean;
        }
        if !self.rebuild && other.rebuild {
            self.rebuild = other.rebuild;
        }
        if self.relative_insert_prefix.is_none() && other.relative_insert_prefix.is_some() {
            self.relative_insert_prefix = other.relative_insert_prefix;
        }
        if self.html_template.is_none() && other.html_template.is_some() {
            self.html_template = other.html_template;
        }
        if self.html_mathmode.is_none() && other.html_mathmode.is_some() {
            self.html_mathmode = other.html_mathmode;
        }
        if !self.html_embed_svg && other.html_embed_svg {
            self.html_embed_svg = other.html_embed_svg;
        }
    }
}
