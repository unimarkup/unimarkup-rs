use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
macro_rules! csl_files {
    ($($name:ident, $path:literal)|+) => {
        $(

        pub const $name: &'static str = include_str!($path);
        )+
    }
}

csl_files!(
    CSL_DE_DE_LOCALE, "../../../csl_locales/locales-de-DE.xml" |
    CSL_AR_LOCALE, "../../../csl_locales/locales-ar.xml" |
    CSL_DE_AT_LOCALE, "../../../csl_locales/locales-de-AT.xml" |
    CSL_EN_GB_LOCALE, "../../../csl_locales/locales-en-GB.xml" |
    CSL_EN_US_LOCALE, "../../../csl_locales/locales-en-US.xml" |
    CSL_ES_ES_LOCALE, "../../../csl_locales/locales-es-ES.xml" |
    CSL_FR_FR_LOCALE, "../../../csl_locales/locales-fr-FR.xml" |
    CSL_HI_IN_LOCALE, "../../../csl_locales/locales-hi-IN.xml" |
    CSL_ZH_CN_LOCALE, "../../../csl_locales/locales-zh-CN.xml"
);

pub fn get_locale_string(paths: HashSet<PathBuf>) -> String {
    let mut default_string = String::from("en-US");
    let mut default_string_set = false;
    for path in paths {
        if path.is_file() {
            return fs::read_to_string(path.into_os_string()).expect("Reading the locale file failed");
        }
        if !default_string_set {
            default_string = String::from(path.to_str().expect("Converting the path to a string failed"));
            default_string_set = true;

        }
    }
    return match default_string.as_str() {
        "de-DE" => String::from(CSL_DE_DE_LOCALE),
        "ar" => String::from(CSL_AR_LOCALE),
        "de-AT" => String::from(CSL_DE_AT_LOCALE),
        "en-GB" => String::from(CSL_EN_GB_LOCALE),
        "en-US" => String::from(CSL_EN_US_LOCALE),
        "es-ES" => String::from(CSL_ES_ES_LOCALE),
        "fr-FR" => String::from(CSL_FR_FR_LOCALE),
        "hi-IN" => String::from(CSL_HI_IN_LOCALE),
        "zh-CN" => String::from(CSL_ZH_CN_LOCALE),
        _ => String::from(CSL_EN_US_LOCALE)
    };
}

csl_files!(
    AMERICAN_MEDICAL_ASSOCIATION, "../../../csl_styles/american-medical-association.csl" |
    APA, "../../../csl_styles/apa.csl" |
    BLUEBOOK_INLINE, "../../../csl_styles/bluebook-inline.csl" |
    CHICAGO_FULLNOTE_BIBLIOGRAPHY, "../../../csl_styles/chicago-fullnote-bibliography.csl" |
    COUNCIL_OF_SCIENCE_EDITORS, "../../../csl_styles/council-of-science-editors.csl" |
    HARVARD_CITE_THEM_RIGHT, "../../../csl_styles/harvard-cite-them-right.csl" |
    IEEE, "../../../csl_styles/ieee.csl" |
    TURABIAN_AUTHOR_DATE, "../../../csl_styles/turabian-author-date.csl"
);

pub fn get_style_string(path: PathBuf) -> String {
    return if path.is_file() {
        fs::read_to_string(path.into_os_string()).expect("Reading the style file failed")
    } else {
        match path.to_str().expect("The style could not be converted to a string") {
            "american-medical-association" => String::from(AMERICAN_MEDICAL_ASSOCIATION),
            "apa" => String::from(APA),
            "bluebook-inline" => String::from(BLUEBOOK_INLINE),
            "chicago-fullnote-bibliography" => String::from(CHICAGO_FULLNOTE_BIBLIOGRAPHY),
            "council-of-science-editors" => String::from(COUNCIL_OF_SCIENCE_EDITORS),
            "harvard-cite-them-right" => String::from(HARVARD_CITE_THEM_RIGHT),
            "ieee" => String::from(IEEE),
            "turabian-author-date" => String::from(TURABIAN_AUTHOR_DATE),
            _ => String::from(IEEE)
        }
    }
}