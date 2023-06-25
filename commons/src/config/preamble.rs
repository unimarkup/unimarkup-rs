use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::{Path, PathBuf},
};

use clap::Args;
use icu_datagen::{all_keys, Out, SourceData};
use icu_locid::{langid, LanguageIdentifier, Locale};

use icu_provider::{BufferProvider, DataRequest};
use logid::{err, logging::event_entry::AddonKind, pipe};
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
    #[arg(long, value_parser = locale::clap::parse_locale, default_value = "en-US")]
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
        let mut locales: Vec<_> = std::iter::once(&self.lang)
            .chain(self.langs.iter())
            .collect();

        locales.sort_by_key(|locale| locale.to_string());
        locales.dedup();

        if self.locales_file.is_none() {
            let allowed_locales = [
                langid!("en"),
                langid!("en-US"),
                langid!("en-UK"),
                langid!("de"),
                langid!("de-DE"),
                langid!("de-AT"),
                langid!("bs"),
                langid!("bs-BA"),
            ];

            if !locales
                .iter()
                .all(|langid| allowed_locales.contains(&langid.id))
            {
                return err!(
                    ConfigErr::BadLocaleUsed,
                    &format!(
                        "{} locale(s) not supported by default. Only the following locales are allowed: {}.",
                        locales
                            .iter()
                            .filter(|l| !allowed_locales.contains(&l.id))
                            .map(|langid| langid.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                        allowed_locales
                            .iter()
                            .map(|langid| langid.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    add: AddonKind::Info(
                        String::from(
                            "Use --locales-file (and --download-locales) when using non-default locales."
                        )
                    )
                );
            }
        }

        let blob = self.get_blob();

        // check if it loads
        let provider =
            icu_provider_blob::BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
                .map_err(|_| {
                    pipe!(
                        ConfigErr::InvalidFile,
                        &format!(
                            "Failed to read locales file: {}",
                            self.locales_file.as_ref().unwrap().to_string_lossy()
                        )
                    )
                })?;

        let key = icu_datagen::key("decimal/symbols@1").expect("decimal/symbols@1 is a valid key.");
        for locale in locales {
            let req = DataRequest {
                locale: &(locale).into(),
                metadata: Default::default(),
            };

            if provider.load_buffer(key, req).is_err() {
                logid::log!(
                    ConfigErr::LocaleMissingKeys(locale.id.language.to_string()),
                    add: AddonKind::Info(
                        format!(
                            "Locale {} is not contained in the provided file. Trying to use fallback locale '{}'.",
                            locale,
                            locale.id.language
                        )
                    )
                );

                let locale = &Locale::from(locale.id.language).into();
                let req = DataRequest {
                    locale,
                    metadata: Default::default(),
                };
                provider.load_buffer(key, req).map_err(|err| {
                    pipe!(
                        ConfigErr::BadLocaleUsed,
                        &format!(
                            "Could not find locale '{}' in data file. Cause: {}",
                            locale, err
                        )
                    )
                })?;
            }
        }

        Ok(())
    }
}

impl I18n {
    pub(crate) fn get_blob(&self) -> Vec<u8> {
        let mut locales: Vec<_> = std::iter::once(&self.lang)
            .chain(self.langs.iter())
            .map(|lang| lang.id.clone())
            .collect();

        // extend with languages without region. Example: bs-BA -> bs
        locales.extend(
            locales
                .clone()
                .iter()
                .map(|locale| LanguageIdentifier::from(locale.language)),
        );

        locales.sort_by_key(LanguageIdentifier::to_string);
        locales.dedup();

        let blob = 'find_file: {
            if let Some(file_path) = &self.locales_file {
                if !file_path.exists()
                    && self.download
                    && self.try_download_icu_file(file_path, &locales).is_err()
                {
                    break 'find_file None;
                }

                match std::fs::read(file_path) {
                    Ok(file) => break 'find_file Some(file),
                    Err(err) => {
                        logid::log!(
                            ConfigErr::InvalidFile,
                            &format!(
                                "Locales file not found: {}. Cause: {}. Using default locales file.",
                                file_path.to_string_lossy(),
                                err
                            )
                        );
                        break 'find_file None;
                    }
                }
            }

            None
        };

        blob.unwrap_or_else(|| Vec::from(include_bytes!("../../locale/data.postcard").as_slice()))
    }

    fn try_download_icu_file(
        &self,
        file_path: impl AsRef<Path>,
        locales: &[LanguageIdentifier],
    ) -> Result<(), ConfigErr> {
        let f = File::create(&file_path).map_err(|_| {
            pipe!(
                ConfigErr::FileCreate,
                &format!(
                    "Failed to create locales file: {}",
                    file_path.as_ref().to_string_lossy()
                )
            )
        })?;

        let out = vec![Out::Blob(Box::new(f))];
        icu_datagen::datagen(
            Some(locales),
            &all_keys(),
            &SourceData::latest_tested(),
            out,
        )
        .map_err(|err| {
            pipe!(
                ConfigErr::LocaleDownload,
                &format!(
                    "Failed to download locales file: {}. Cause: {}",
                    file_path.as_ref().to_string_lossy(),
                    err,
                )
            )
        })
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
                    &format!("Citation Style Language file not found: {:?}", file)
                );
            }
        }

        for reference in &self.references {
            if !reference.exists() {
                return err!(
                    ConfigErr::InvalidFile,
                    &format!("Bibliography references file not found: {:?}", reference)
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
                    &format!("Font file not found: {:?}", font)
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
                    &format!("Favicon file not found: {:?}", fav)
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
