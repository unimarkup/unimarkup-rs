//! [`preamble`](crate::frontend::preamble) is the module which implements

use crate::{config::Config, frontend::parser::Rule, um_error::UmError};
use pest::iterators::Pair;

/// [`parse_preamble`] parses the preamble and serializes depending if JSON or YAML.#
/// Then compares the config of the preamble with the CLI and complement arguments from the preamble to the CLI arguments
pub fn parse_preamble(pairs: Pair<Rule>, config: &mut Config) -> Result<(), UmError> {
    let preamble = pairs.into_inner().next().unwrap();

    if preamble.as_rule() == Rule::json_body {
        if let Ok(serialized) = serde_json::from_str::<Config>(preamble.as_str()) {
            compare_configs(serialized, config);
        } else {
            return Err(UmError::custom_pest_error(
                "Expected JSON",
                preamble.as_span(),
            ));
        }
    }
    if preamble.as_rule() == Rule::yaml_body {
        if let Ok(serialized) = serde_yaml::from_str::<Config>(preamble.as_str()) {
            compare_configs(serialized, config);
        } else {
            return Err(UmError::custom_pest_error(
                "Expected YAML",
                preamble.as_span(),
            ));
        }
    }
    Ok(())
}

fn compare_configs(preamble: Config, cli: &mut Config) {
    if cli.out_file.is_none() && preamble.out_file.is_some() {
        cli.out_file = preamble.out_file;
    }
    if cli.out_formats.is_none() && preamble.out_formats.is_some() {
        cli.out_formats = preamble.out_formats;
    }
    if cli.insert_paths.is_none() && preamble.insert_paths.is_some() {
        cli.insert_paths = preamble.insert_paths;
    }
    if cli.dot_unimarkup.is_none() && preamble.dot_unimarkup.is_some() {
        cli.dot_unimarkup = preamble.dot_unimarkup;
    }
    if cli.theme.is_none() && preamble.theme.is_some() {
        cli.theme = preamble.theme;
    }
    if cli.flags.is_none() && preamble.flags.is_some() {
        cli.flags = preamble.flags;
    }
    if cli.enable_elements.is_none() && preamble.enable_elements.is_some() {
        cli.enable_elements = preamble.enable_elements;
    }
    if cli.disable_elements.is_none() && preamble.disable_elements.is_some() {
        cli.disable_elements = preamble.disable_elements;
    }
    if cli.citation_style.is_none() && preamble.citation_style.is_some() {
        cli.citation_style = preamble.citation_style;
    }
    if cli.references.is_none() && preamble.references.is_some() {
        cli.references = preamble.references;
    }
    if cli.fonts.is_none() && preamble.fonts.is_some() {
        cli.fonts = preamble.fonts;
    }
    if !cli.overwrite_out_files && preamble.overwrite_out_files {
        cli.overwrite_out_files = preamble.overwrite_out_files;
    }
    if !cli.clean && preamble.clean {
        cli.clean = preamble.clean;
    }
    if !cli.rebuild && preamble.rebuild {
        cli.rebuild = preamble.rebuild;
    }
    if cli.relative_insert_prefix.is_none() && preamble.relative_insert_prefix.is_some() {
        cli.relative_insert_prefix = preamble.relative_insert_prefix;
    }
    if cli.html_template.is_none() && preamble.html_template.is_some() {
        cli.html_template = preamble.html_template;
    }
    if cli.html_mathmode.is_none() && preamble.html_mathmode.is_some() {
        cli.html_mathmode = preamble.html_mathmode;
    }
    if !cli.html_embed_svg && preamble.html_embed_svg {
        cli.html_embed_svg = preamble.html_embed_svg;
    }
}

#[test]
fn syntax_error_json() {}
