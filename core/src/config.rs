//! Structs and functions for parsing and generating the Unimarkup configuration.

use std::path::PathBuf;

use clap::{crate_version, ArgEnum, Parser};
use logid::{
    capturing::{LogIdTracing, MappedLogId},
    log_id::LogId,
};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{
    elements::types::ElementType,
    log_id::{ConfigErrLogId, CORE_LOG_ID_MAP},
};

const UNIMARKUP_NAME: &str = "Unimarkup";

/// Config contains the possible options for the Unimarkup configuration.
#[derive(Debug, PartialEq, Eq, Clone, Parser, Default, Serialize, Deserialize)]
#[clap(name = UNIMARKUP_NAME, version = crate_version!())]
pub struct Config {
    /// The filename of the Unimarkup file that is used as root for rendering.
    #[clap(
        index = 1,
        value_name = "UM-FILE",
        required = true,
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(skip)]
    pub um_file: PathBuf,

    /// The filename without filetype to be used for output filenames. If a path is part of the filename, output files are saved at the given path.
    #[clap(
        index = 2,
        value_name = "OUTPUT-FILE",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "OUTPUT-FILE")]
    #[serde(default)]
    pub out_file: Option<PathBuf>,

    /// Set output formats the Unimarkup document should be rendered to. Set outputs are also treated as flags inside the Unimarkup document.
    #[clap(
        name = "output-formats",
        display_order = 1,
        long = "output-formats",
        alias = "formats",
        takes_value = true,
        use_value_delimiter = true,
        arg_enum
    )]
    #[serde(alias = "output-formats")]
    #[serde(alias = "formats")]
    #[serde(default)]
    pub out_formats: Option<Vec<OutputFormat>>,

    /// Set paths that are searched for relative file and image inserts.
    #[clap(
        display_order = 2,
        long = "insert-paths",
        takes_value = true,
        use_value_delimiter = true,
        parse(from_os_str)
    )]
    #[serde(alias = "insert-paths")]
    #[serde(default)]
    pub insert_paths: Option<Vec<PathBuf>>,

    /// Set the path to a directory that is used for default preamble and theme settings.
    /// The intermediate form of rendered documents will also be stored at this path.
    #[clap(
        display_order = 3,
        long = "dot-unimarkup",
        alias = "config",
        takes_value = true,
        env = "UNIMARKUP_CONFIG",
        parse(from_os_str)
    )]
    #[serde(alias = "dot-unimarkup")]
    #[serde(alias = "config")]
    #[serde(default)]
    pub dot_unimarkup: Option<PathBuf>,

    /// Set a Unimarkup theme file to be used for rendering.
    #[clap(
        display_order = 5,
        short = 't',
        long = "theme",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "theme")]
    #[serde(default)]
    pub theme: Option<PathBuf>,

    /// Set flags that will be set for rendering.
    #[clap(
        display_order = 6,
        short = 'f',
        long = "flags",
        takes_value = true,
        use_value_delimiter = true
    )]
    #[serde(alias = "flags")]
    #[serde(default)]
    pub flags: Option<Vec<String>>,

    /// Explicitly set Unimarkup block elements that can be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are disabled.
    #[clap(
        display_order = 7,
        long = "enable-elements",
        takes_value = true,
        use_value_delimiter = true,
        arg_enum
    )]
    #[serde(alias = "enable-elements")]
    #[serde(default)]
    pub enable_elements: Option<Vec<ElementType>>,

    /// Explicitly set Unimarkup block elements that can NOT be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are enabled.
    #[clap(
        display_order = 8,
        long = "disable-elements",
        takes_value = true,
        use_value_delimiter = true,
        arg_enum
    )]
    #[serde(alias = "disable-elements")]
    #[serde(default)]
    pub disable_elements: Option<Vec<ElementType>>,

    /// Set citation style sheet that is used to process referenced literature
    #[clap(
        display_order = 30,
        long = "citation-style",
        alias = "csl",
        takes_value = true,
        requires = "references",
        parse(from_os_str)
    )]
    #[serde(alias = "citation-style")]
    #[serde(alias = "csl")]
    #[serde(default)]
    pub citation_style: Option<PathBuf>,

    /// Set one or more reference files in bibtex or JSON format to use them with literature referencing.
    #[clap(
        display_order = 31,
        long = "references",
        alias = "refs",
        takes_value = true,
        use_value_delimiter = true,
        requires = "citation-style",
        parse(from_os_str)
    )]
    #[serde(alias = "references")]
    #[serde(alias = "refs")]
    #[serde(default)]
    pub references: Option<Vec<PathBuf>>,

    /// Set ttf or woff fonts to be able to use them for rendering
    #[clap(
        display_order = 10,
        long = "fonts",
        alias = "ttf",
        alias = "woff",
        takes_value = true,
        use_value_delimiter = true,
        parse(from_os_str)
    )]
    #[serde(alias = "fonts")]
    #[serde(alias = "ttf")]
    #[serde(alias = "woff")]
    #[serde(default)]
    pub fonts: Option<Vec<PathBuf>>,

    /// Overwrites files set with `out-file` if already existing.
    #[clap(
        display_order = 1,
        short = 'w',
        long = "overwrite-out-files",
        takes_value = false
    )]
    #[serde(alias = "overwrite-out-files")]
    #[serde(default)]
    pub overwrite_out_files: bool,

    /// Deletes all previously rendered documents stored inside the UNIMARKUP_CONFIG path.
    #[clap(display_order = 2, short = 'c', long = "clean", takes_value = false)]
    #[serde(alias = "clean")]
    #[serde(alias = "c")]
    #[serde(default)]
    pub clean: bool,

    /// Ignores all previously rendered documents stored inside the UNIMARKUP_CONFIG path and renders the given Unimarkup file.
    #[clap(display_order = 3, short = 'r', long = "rebuild", takes_value = false)]
    #[serde(alias = "rebuild")]
    #[serde(default)]
    pub rebuild: bool,

    /// Set if preamble of given Unimarkup file is replaced with the given arguments.
    /// If not set, given arguments overwrite the corresponding preamble settings, but other settings are still used.
    #[clap(
        display_order = 20,
        long = "replace-preamble",
        requires = "output-formats",
        takes_value = false
    )]
    #[serde(skip)]
    pub replace_preamble: bool,

    /// This prefix will be set before inserts in the rendered document to inserts that use relative paths.
    /// Note: During rendering, the original relative path is taken.
    #[clap(
        display_order = 20,
        long = "relative-insert-prefix",
        alias = "insert-prefix",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "relative-insert-prefix")]
    #[serde(alias = "insert-prefix")]
    #[serde(default)]
    pub relative_insert_prefix: Option<PathBuf>,

    /// Set a template html file with `{{ head }}` set inside the `head` element and `{{ body }}` set inside the body element.
    /// Styling, fonts and scripts will be inserted at `{{ head }}` and the rendered Unimarkup content is placed inside `{{ body }}`.
    /// Optionally, `{{ toc }}` can be set to get the table of contents (Note: This will not remove a rendered table of contents inside the rendered Unimarkup content if present).
    #[clap(
        display_order = 40,
        long = "html-template",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "html-template")]
    #[serde(default)]
    pub html_template: Option<PathBuf>,

    /// Set the mathmode of MathJax to be used for rendered html documents.
    #[clap(
        display_order = 41,
        long = "html-mathmode",
        takes_value = true,
        arg_enum
    )]
    #[serde(alias = "html-mathmode")]
    #[serde(default)]
    pub html_mathmode: Option<HtmlMathmode>,

    /// Set if svgs should be embedded into html instead of inserted as regular images.
    #[clap(display_order = 40, long = "html-embed-svg", takes_value = false)]
    #[serde(alias = "html-embed-svg")]
    #[serde(default)]
    pub html_embed_svg: bool,
}

/// Possible output formats for a Unimarkup file
#[derive(
    Debug, PartialEq, Eq, Clone, EnumString, ArgEnum, strum_macros::Display, Serialize, Deserialize,
)]
pub enum OutputFormat {
    /// PDF output format
    #[strum(ascii_case_insensitive)]
    Pdf,
    /// HTML output format
    #[strum(ascii_case_insensitive)]
    Html,
    #[strum(ascii_case_insensitive, serialize = "reveal-js")]
    #[clap(alias = "revealjs")]
    /// [revealJs] output format.
    ///
    /// A presentation framework using HTML and Javascript
    ///
    /// [revealJs]: https://revealjs.com/
    RevealJs,
    #[strum(ascii_case_insensitive)]
    /// Intermediate representation of the Unimarkup document.
    Intermediate,
}

/// Possible modes for rendering math formulas in HTML
#[derive(
    Debug, PartialEq, Eq, Clone, EnumString, ArgEnum, strum_macros::Display, Serialize, Deserialize,
)]
pub enum HtmlMathmode {
    /// Render math as SVG
    #[strum(ascii_case_insensitive)]
    Svg,
    /// Embed MathJax
    #[strum(ascii_case_insensitive)]
    Embed,
    /// Use CDN (Content Delivery Network) for MathJax (requires online connection to view math formulas in the output HTML)
    #[strum(ascii_case_insensitive)]
    Cdn,
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

    /// [`validate_config`] validates if file and paths exist and if config does not contradict itself
    pub fn validate_config(&mut self) -> Result<(), MappedLogId> {
        if let Some(ref file) = self.out_file {
            if file.exists() && !self.overwrite_out_files {
                return Err((ConfigErrLogId::InvalidConfig as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    "Option `overwrite-out-files` must be `true` if output file exists.",
                    file!(),
                    line!(),
                ));
            }
        }
        if let Some(ref paths) = self.insert_paths {
            for path in paths {
                if !path.exists() {
                    return Err((ConfigErrLogId::InvalidPath as LogId).set_event_with(
                        &CORE_LOG_ID_MAP,
                        &format!("Invalid path given for `insert-paths`: {:?}", path),
                        file!(),
                        line!(),
                    ));
                }
            }
        }
        if let Some(ref path) = self.dot_unimarkup {
            if !path.is_dir() {
                return Err((ConfigErrLogId::InvalidPath as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Invalid path given for `dot-unimarkup`: {:?}", path),
                    file!(),
                    line!(),
                ));
            }
        }
        if let Some(ref file) = self.theme {
            if !file.exists() {
                return Err((ConfigErrLogId::InvalidFile as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Invalid file given for `theme`: {:?}", file),
                    file!(),
                    line!(),
                ));
            }
        }
        if let Some(ref file) = self.citation_style {
            if !file.exists() {
                return Err((ConfigErrLogId::InvalidFile as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Invalid file given for `citation-style`: {:?}", file),
                    file!(),
                    line!(),
                ));
            }
        }
        if let Some(ref files) = self.references {
            for file in files {
                if !file.exists() {
                    return Err((ConfigErrLogId::InvalidFile as LogId).set_event_with(
                        &CORE_LOG_ID_MAP,
                        &format!("Invalid file given for `references`: {:?}", file),
                        file!(),
                        line!(),
                    ));
                }
            }
        }
        if let Some(ref files) = self.fonts {
            for file in files {
                if !file.exists() {
                    return Err((ConfigErrLogId::InvalidFile as LogId).set_event_with(
                        &CORE_LOG_ID_MAP,
                        &format!("Invalid file given for `fonts`: {:?}", file),
                        file!(),
                        line!(),
                    ));
                }
            }
        }
        if let Some(ref file) = self.html_template {
            if !file.exists() {
                return Err((ConfigErrLogId::InvalidFile as LogId).set_event_with(
                    &CORE_LOG_ID_MAP,
                    &format!("Invalid file given for `html-template`: {:?}", file),
                    file!(),
                    line!(),
                ));
            }
        }
        if !self.um_file.exists() {
            return Err((ConfigErrLogId::InvalidFile as LogId).set_event_with(
                &CORE_LOG_ID_MAP,
                "Set `um-file` does not exist!",
                file!(),
                line!(),
            ));
        }

        Ok(())
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__validate__valid_config() {
        let mut cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            "--dot-unimarkup=tests/test_files/",
            "tests/test_files/frontend/heading1.um",
        ]);

        let result = cfg.validate_config();
        assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    }

    #[should_panic]
    #[test]
    fn test__validate__invalid_config() {
        let mut cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            //invalid attribute "shouldfail" on purpose
            "--dot-unimarkup=shouldfail",
            "tests/test_files/frontend/heading1.um",
        ]);

        cfg.validate_config().unwrap();
    }

    #[test]
    fn test__validate__valid_multi_path_config() {
        let mut cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            "--dot-unimarkup=tests/test_files/",
            "--insert-paths=tests/test_files/,tests/test_files/",
            "tests/test_files/frontend/heading1.um",
        ]);

        let result = cfg.validate_config();
        assert!(result.is_ok(), "Cause: {:?}", result.unwrap_err());
    }

    #[should_panic]
    #[test]
    fn test__validate__invalid_multi_path_config() {
        let mut cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            "--dot-unimarkup=tests/test_files/",
            //invalid attribute "shouldfail" on purpose
            "--insert-paths=shouldfail,tests/test_files/",
            "tests/test_files/frontend/heading1.um",
        ]);

        cfg.validate_config().unwrap();
    }
}
