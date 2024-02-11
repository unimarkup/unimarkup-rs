use crate::log_id::GeneralWarning;
use logid::log;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use unimarkup_commons::config::icu_locid::Locale;
macro_rules! csl_files {
    ($($name:ident, $path:literal)|+) => {
        $(

        pub const $name: &'static str = include_str!($path);
        )+
    }
}

csl_files!(
    CSL_DE_DE_LOCALE,
    "../../../csl_locales/locales-de-DE.xml" | CSL_AR_LOCALE,
    "../../../csl_locales/locales-ar.xml" | CSL_DE_AT_LOCALE,
    "../../../csl_locales/locales-de-AT.xml" | CSL_EN_GB_LOCALE,
    "../../../csl_locales/locales-en-GB.xml" | CSL_EN_US_LOCALE,
    "../../../csl_locales/locales-en-US.xml" | CSL_ES_ES_LOCALE,
    "../../../csl_locales/locales-es-ES.xml" | CSL_FR_FR_LOCALE,
    "../../../csl_locales/locales-fr-FR.xml" | CSL_HI_IN_LOCALE,
    "../../../csl_locales/locales-hi-IN.xml" | CSL_ZH_CN_LOCALE,
    "../../../csl_locales/locales-zh-CN.xml"
);

fn get_cached_locale_string(locale: Locale) -> &'static str {
    return match locale.to_string().as_str() {
        "de-DE" => CSL_DE_DE_LOCALE,
        "ar" => CSL_AR_LOCALE,
        "de-AT" => CSL_DE_AT_LOCALE,
        "en-GB" => CSL_EN_GB_LOCALE,
        "en-US" => CSL_EN_US_LOCALE,
        "es-ES" => CSL_ES_ES_LOCALE,
        "fr-FR" => CSL_FR_FR_LOCALE,
        "hi-IN" => CSL_HI_IN_LOCALE,
        "zh-CN" => CSL_ZH_CN_LOCALE,
        _ => CSL_EN_US_LOCALE,
    };
}

pub fn get_locale_string(doc_locale: Locale, paths: HashMap<Locale, PathBuf>) -> String {
    if let Some(path) = paths.get(&doc_locale) {
        if path.is_file() {
            if let Ok(csl_locale) = fs::read_to_string(path) {
                return csl_locale;
            }
            log!(
                GeneralWarning::FileRead,
                format!("Could not read locale file: '{:?}'", &path),
            );
        }
    }
    get_cached_locale_string(doc_locale).to_string()
}

csl_files!(
    AMERICAN_MEDICAL_ASSOCIATION,
    "../../../csl_styles/american-medical-association.csl" | APA,
    "../../../csl_styles/apa.csl" | BLUEBOOK_INLINE,
    "../../../csl_styles/bluebook-inline.csl" | CHICAGO_FULLNOTE_BIBLIOGRAPHY,
    "../../../csl_styles/chicago-fullnote-bibliography.csl" | COUNCIL_OF_SCIENCE_EDITORS,
    "../../../csl_styles/council-of-science-editors.csl" | HARVARD_CITE_THEM_RIGHT,
    "../../../csl_styles/harvard-cite-them-right.csl" | IEEE,
    "../../../csl_styles/ieee.csl" | TURABIAN_AUTHOR_DATE,
    "../../../csl_styles/turabian-author-date.csl"
);

pub fn get_style_string(path: PathBuf) -> String {
    if path.is_file() {
        return match fs::read_to_string(&path) {
            Ok(csl_style) => csl_style,
            Err(_) => {
                log!(
                    GeneralWarning::FileRead,
                    format!("Could not read style file: '{:?}'", path.clone()),
                );
                IEEE.to_string()
            }
        };
    }
    match path.to_str().unwrap_or("ieee") {
        "american-medical-association" => AMERICAN_MEDICAL_ASSOCIATION.to_string(),
        "apa" => APA.to_string(),
        "bluebook-inline" => BLUEBOOK_INLINE.to_string(),
        "chicago-fullnote-bibliography" => CHICAGO_FULLNOTE_BIBLIOGRAPHY.to_string(),
        "council-of-science-editors" => COUNCIL_OF_SCIENCE_EDITORS.to_string(),
        "harvard-cite-them-right" => HARVARD_CITE_THEM_RIGHT.to_string(),
        "ieee" => IEEE.to_string(),
        "turabian-author-date" => TURABIAN_AUTHOR_DATE.to_string(),
        _ => {
            log!(
                GeneralWarning::UnsupportedCslStyle,
                format!("The csl style '{:?}' is not supported", path.to_str()),
            );
            IEEE.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::html::citeproc::csl_files::{get_locale_string, get_style_string};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::str::FromStr;
    use unimarkup_commons::config::icu_locid::locale;

    #[test]
    fn get_locale_string_not_in_hashmap() {
        let mut map = HashMap::new();
        map.insert(locale!("fr"), PathBuf::from_str("path/to/fr").unwrap());
        let actual = get_locale_string(locale!("de-DE"), map);
        assert!(actual.contains("xml:lang=\"de-DE\""));
    }

    #[test]
    fn get_locale_string_path_not_found() {
        let mut map = HashMap::new();
        map.insert(
            locale!("de-DE"),
            PathBuf::from_str("invalid/path/to/de-DE").unwrap(),
        );
        let actual = get_locale_string(locale!("de-DE"), map);
        assert!(actual.contains("xml:lang=\"de-DE\""));
    }

    #[test]
    fn get_locale_string_from_path() {
        let mut map = HashMap::new();
        map.insert(
            locale!("nl-NL"),
            PathBuf::from_str("./src/html/citeproc/test_files/locales-nl-NL.xml").unwrap(),
        );
        let actual = get_locale_string(locale!("nl-NL"), map);
        assert!(actual.contains("xml:lang=\"nl-NL\""));
    }

    #[test]
    fn get_locale_string_default() {
        let map = HashMap::new();
        let actual = get_locale_string(locale!("nl-NL"), map);
        assert!(actual.contains("xml:lang=\"en-US\""));
    }

    #[test]
    fn get_style_string_file_found() {
        let path = PathBuf::from_str("./csl_styles/apa.csl").unwrap();
        let actual = get_style_string(path);
        assert!(actual.contains("<title>American Psychological Association 7th edition</title>"));
    }

    #[test]
    fn get_style_string_file_not_found() {
        let path = PathBuf::from_str("invalid/path/file.csl").unwrap();
        let actual = get_style_string(path);
        assert!(actual.contains("<title>IEEE</title>"));
    }

    #[test]
    fn get_style_string_no_file() {
        let path = PathBuf::from_str("harvard-cite-them-right").unwrap();
        let actual = get_style_string(path);
        assert!(actual.contains("<title>Cite Them Right 12th edition - Harvard</title>"));
    }
}
