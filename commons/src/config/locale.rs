pub mod serde {
    use icu_locid::Locale;
    use serde::{Deserialize, Deserializer, Serializer};

    pub mod single {
        use super::*;

        pub fn serialize<S>(locale: &Locale, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let locale = locale.to_string();
            serializer.serialize_str(&locale)
        }

        // The signature of a deserialize_with function must follow the pattern:
        //
        //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
        //    where
        //        D: Deserializer<'de>
        //
        // although it may also be generic over the output types T.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Locale, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        }
    }

    pub mod multiple {
        use super::*;
        use crate::config::log_id::ConfigWarning;
        use logid::log;
        use serde::de::{SeqAccess, Visitor};
        use serde::ser::SerializeSeq;
        use std::collections::HashSet;

        pub fn serialize<S>(locales: &HashSet<Locale>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(locales.len()))?;
            for locale in locales {
                seq.serialize_element(&locale.to_string())?;
            }
            seq.end()
        }

        // The signature of a deserialize_with function must follow the pattern:
        //
        //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
        //    where
        //        D: Deserializer<'de>
        //
        // although it may also be generic over the output types T.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Locale>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(LocaleSeqVisitor {})
        }

        struct LocaleSeqVisitor {}

        impl<'de> Visitor<'de> for LocaleSeqVisitor {
            type Value = HashSet<Locale>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Sequence of locales.")
            }
            fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: SeqAccess<'de>,
            {
                let mut set = HashSet::with_capacity(access.size_hint().unwrap_or(0));
                while let Some(value) = access.next_element::<String>()? {
                    match value.parse::<Locale>() {
                        Ok(locale) => {
                            set.insert(locale);
                        }
                        Err(e) => {
                            log!(
                                ConfigWarning::InvalidOutputLang,
                                format!("Could not parse the output language to locale with error: '{}'", e)
                            );
                        }
                    }
                    let locale = value.parse::<Locale>().map_err(serde::de::Error::custom)?;
                    set.insert(locale);
                }
                Ok(set)
            }
        }
    }

    pub mod optional {
        use super::*;

        pub fn serialize<S>(locale: &Option<Locale>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match locale.as_ref().map(|l| l.to_string()) {
                Some(locale) => serializer.serialize_some(&locale),
                None => serializer.serialize_none(),
            }
        }

        // The signature of a deserialize_with function must follow the pattern:
        //
        //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
        //    where
        //        D: Deserializer<'de>
        //
        // although it may also be generic over the output types T.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Locale>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match s.parse() {
                Ok(locale) => Ok(Some(locale)),
                Err(_) => Ok(None),
            }
        }
    }

    pub mod hashmap {
        use super::*;
        use serde::de::{MapAccess, Visitor};
        use serde::ser::SerializeMap;
        use std::collections::HashMap;
        use std::path::PathBuf;

        pub fn serialize<S>(
            locales_map: &HashMap<Locale, PathBuf>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(locales_map.len()))?;
            for (locale, path) in locales_map {
                map.serialize_entry(&locale.to_string(), path)?;
            }
            map.end()
        }

        // The signature of a deserialize_with function must follow the pattern:
        //
        //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
        //    where
        //        D: Deserializer<'de>
        //
        // although it may also be generic over the output types T.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<Locale, PathBuf>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(LocaleMapVisitor {})
        }
        struct LocaleMapVisitor {}
        impl<'de> Visitor<'de> for LocaleMapVisitor {
            type Value = HashMap<Locale, PathBuf>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Map with locale as key and file path as value.")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));
                while let Some((key, value)) = access.next_entry::<String, PathBuf>()? {
                    let locale = key.parse::<Locale>().map_err(serde::de::Error::custom)?;
                    map.insert(locale, value);
                }
                Ok(map)
            }
        }
    }
}

pub mod clap {
    pub fn parse_locale(input: &str) -> Result<icu_locid::Locale, clap::Error> {
        input.parse().map_err(|err| {
            clap::Error::raw(
                clap::error::ErrorKind::InvalidValue,
                format!("Invalid locale given: {:?}", err),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::locale::serde::{hashmap, multiple};
    use icu_locid::{locale, Locale};
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[derive(Serialize, Deserialize)]
    struct LocalesStruct {
        #[serde(with = "multiple")]
        locales: HashSet<Locale>,
    }

    #[derive(Serialize, Deserialize)]
    struct LocalePathBufStruct {
        #[serde(with = "hashmap")]
        map: HashMap<Locale, PathBuf>,
    }

    #[test]
    fn serialize_multiple() {
        let mut locales = HashSet::new();
        locales.insert(locale!("de"));
        locales.insert(locale!("en"));

        let locales_struct = LocalesStruct { locales };

        let actual = serde_yaml::to_string(&locales_struct).unwrap();
        assert!(
            actual == "---\nlocales:\n  - en\n  - de\n"
                || actual == "---\nlocales:\n  - de\n  - en\n"
        );
    }

    #[test]
    fn deserialize_multiple() {
        let serialized = "locales:\n  - en\n  - de";
        let actual: LocalesStruct = serde_yaml::from_str(serialized).unwrap();

        let locales_vec: Vec<Locale> = actual.locales.into_iter().collect();
        assert_eq!(locales_vec.len(), 2);
        assert_ne!(locales_vec[0], locales_vec[1]);
        assert!(locales_vec[0] == locale!("de") || locales_vec[0] == locale!("en"));
        assert!(locales_vec[1] == locale!("de") || locales_vec[1] == locale!("en"));
    }

    #[test]
    fn serialize_hashmap() {
        let mut map = HashMap::new();
        map.insert(locale!("de"), PathBuf::from_str("path/to/de").unwrap());
        map.insert(locale!("en"), PathBuf::from_str("path/to/en").unwrap());

        let locale_pathbuf_struct = LocalePathBufStruct { map };

        let actual = serde_yaml::to_string(&locale_pathbuf_struct).unwrap();
        assert!(
            actual == "---\nmap:\n  en: path/to/en\n  de: path/to/de\n"
                || actual == "---\nmap:\n  de: path/to/de\n  en: path/to/en\n"
        );
    }

    #[test]
    fn deserialize_hashmap() {
        let serialized = "map:\n  de: path/to/de\n  en: path/to/en";
        let actual: LocalePathBufStruct = serde_yaml::from_str(serialized).unwrap();

        let locales_map: Vec<(Locale, PathBuf)> = actual.map.into_iter().collect();

        assert_eq!(locales_map.len(), 2);
        assert_ne!(locales_map[0], locales_map[1]);
        assert!(
            (locales_map[0].0 == locale!("de")
                && locales_map[0].1 == PathBuf::from_str("path/to/de").unwrap())
                || (locales_map[0].0 == locale!("en")
                    && locales_map[0].1 == PathBuf::from_str("path/to/en").unwrap())
        );
        assert!(
            (locales_map[1].0 == locale!("de")
                && locales_map[1].1 == PathBuf::from_str("path/to/de").unwrap())
                || (locales_map[1].0 == locale!("en")
                    && locales_map[1].1 == PathBuf::from_str("path/to/en").unwrap())
        );
    }
}
