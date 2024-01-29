use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Args;
use icu_locid::Locale;
use logid::err;
use serde::{Deserialize, Serialize};

use super::{locale, log_id::ConfigErr, parse_to_hashset, ConfigFns, ReplaceIfNone};

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preamble {
    #[command(flatten)]
    #[serde(flatten)]
    pub metadata: Metadata,
    #[command(flatten)]
    #[serde(flatten)]
    pub cite: Citedata,
    #[command(flatten)]
    #[serde(flatten)]
    pub render: RenderConfig,
    #[command(flatten)]
    #[serde(flatten)]
    pub i18n: I18n,
}

impl ConfigFns for Preamble {
    fn merge(&mut self, other: Self) {
        self.metadata.merge(other.metadata);
        self.cite.merge(other.cite);
        self.render.merge(other.render);
        self.i18n.merge(other.i18n);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        self.metadata.validate()?;
        self.cite.validate()?;
        self.render.validate()?;
        self.i18n.validate()
    }
}

pub fn default_locale() -> Locale {
    icu_locid::locale!("en")
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18n {
    #[arg(long, value_parser = locale::clap::parse_locale, default_value = "en")]
    #[serde(with = "locale::serde::single", default = "self::default_locale")]
    pub lang: Locale,

    #[arg(long, value_parser = parse_to_hashset::<Locale>, required = false, default_value = "")]
    #[serde(with = "locale::serde::multiple", default)]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub output_langs: HashSet<Locale>,
}

impl ConfigFns for I18n {
    fn merge(&mut self, other: Self) {
        self.output_langs.extend(other.output_langs);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        // Note: Validity of locales is already ensured by `parse_locale`.

        Ok(())
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderConfig {
    #[arg(long = "ignore-file", value_parser = parse_ignore_file, required = false, default_value = "")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub ignore: HashSet<String>,
    #[arg(long, value_parser = parse_parameter, required = false, default_value = "")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub parameter: HashMap<String, String>,
    #[arg(long)]
    #[serde(default)]
    pub keep_comments: bool,
    #[arg(long)]
    #[serde(default)]
    pub allow_unsafe: bool,
}

impl ConfigFns for RenderConfig {
    fn merge(&mut self, other: Self) {
        self.ignore.extend(other.ignore);
        self.parameter.extend(other.parameter);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        // TODO: validate ignore and parameter syntax
        Ok(())
    }
}

// TODO: Instead of PathBufs, file contents should be parsed on deserialization.
// This makes it easier to access the parsed data without creating another config struct.
// It also makes compiling faster for bad inputs, since it would break before parsing starts.
#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citedata {
    #[arg(long = "citation-style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub references: HashSet<PathBuf>,
}

impl ConfigFns for Citedata {
    fn merge(&mut self, other: Self) {
        self.style.replace_none(other.style);
        self.references.extend(other.references);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        if let Some(file) = &self.style {
            if !file.exists() {
                return err!(
                    ConfigErr::InvalidFile,
                    format!("Citation Style Language file not found: {:?}", file)
                );
            }
        }

        for reference in &self.references {
            if !reference.exists() {
                return err!(
                    ConfigErr::InvalidFile,
                    format!("Bibliography references file not found: {:?}", reference)
                );
            }
        }

        Ok(())
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub authors: HashSet<String>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub fonts: HashSet<PathBuf>,
}

impl ConfigFns for Metadata {
    fn merge(&mut self, other: Self) {
        self.title.replace_none(other.title);
        self.authors.extend(other.authors);
        self.fonts.extend(other.fonts);

        // Note: `base` and `description` must not be merged with sub-configs according to specification.
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        for font in &self.fonts {
            if !font.exists() {
                return err!(
                    ConfigErr::InvalidFile,
                    format!("Font file not found: {:?}", font)
                );
            }
        }

        Ok(())
    }
}

pub fn parse_parameter(_s: &str) -> Result<HashMap<String, String>, clap::Error> {
    //TODO: Implement once parameter parser is implemented

    //TODO: Check for `HtmlSpecificParameter` in given parameters

    Ok(HashMap::default())
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlSpecificParameter {
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    #[serde(default)]
    pub favicons: HashSet<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    #[serde(default)]
    pub keywords: HashSet<String>,
}

impl ConfigFns for HtmlSpecificParameter {
    fn merge(&mut self, other: Self) {
        self.favicons.extend(other.favicons);
        self.keywords.extend(other.keywords);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        for fav in &self.favicons {
            if !fav.exists() {
                return err!(
                    ConfigErr::InvalidFile,
                    format!("Favicon file not found: {:?}", fav)
                );
            }
        }

        Ok(())
    }
}

pub fn parse_ignore_file(_s: &str) -> Result<HashSet<String>, clap::Error> {
    //TODO: Implement once ignore file parser is implemented

    Ok(HashSet::default())
}
