use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Args;
use icu::locid::Locale;
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

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18n {
    #[arg(long, value_parser = locale::clap::parse_locale, default_value = "en")]
    #[serde(with = "locale::serde::single")]
    pub lang: Locale,

    #[arg(long, value_parser = parse_to_hashset::<Locale>, required = false, default_value = "")]
    #[serde(with = "locale::serde::multiple")]
    pub langs: HashSet<Locale>,

    #[arg(long = "locales-file")]
    pub locales_file: Option<PathBuf>,

    #[arg(long = "download-locales")]
    pub download: bool,
}

impl ConfigFns for I18n {
    fn merge(&mut self, other: Self) {
        self.langs.extend(other.langs.into_iter());
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        // Note: Validity of locales is already ensured by `parse_locale`.

        Ok(())
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderConfig {
    #[arg(long = "ignore-file", value_parser = parse_ignore_file, required = false, default_value = "")]
    pub ignore: HashSet<String>,
    #[arg(long, value_parser = parse_parameter, required = false, default_value = "")]
    pub parameter: HashMap<String, String>,
    #[arg(long)]
    pub keep_comments: bool,
    #[arg(long)]
    pub allow_unsafe: bool,
}

impl ConfigFns for RenderConfig {
    fn merge(&mut self, other: Self) {
        self.ignore.extend(other.ignore.into_iter());
        self.parameter.extend(other.parameter.into_iter());
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
    pub style: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    pub references: HashSet<PathBuf>,
}

impl ConfigFns for Citedata {
    fn merge(&mut self, other: Self) {
        self.style.replace_none(other.style);
        self.references.extend(other.references.into_iter());
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
    pub title: Option<String>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub authors: HashSet<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub base: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    pub fonts: HashSet<PathBuf>,
}

impl ConfigFns for Metadata {
    fn merge(&mut self, other: Self) {
        self.title.replace_none(other.title);
        self.authors.extend(other.authors.into_iter());
        self.fonts.extend(other.fonts.into_iter());

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
    pub favicons: HashSet<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub keywords: HashSet<String>,
}

impl ConfigFns for HtmlSpecificParameter {
    fn merge(&mut self, other: Self) {
        self.favicons.extend(other.favicons.into_iter());
        self.keywords.extend(other.keywords.into_iter());
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
