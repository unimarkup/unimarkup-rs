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
        use std::collections::HashSet;

        pub fn serialize<S>(locales: &HashSet<Locale>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut res = String::new();

            for locale in locales.iter() {
                res.push_str(&locale.to_string());
                res.push(',');
            }

            res.pop();

            serializer.serialize_str(&res)
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
            let s = String::deserialize(deserializer)?;

            s.split(',')
                .map(|lang| lang.parse().map_err(serde::de::Error::custom))
                .collect()
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
        use std::collections::HashMap;
        use std::path::PathBuf;
        use serde::ser::SerializeMap;

        pub fn serialize<S>(
            locales_map: &HashMap<Locale, PathBuf>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(locales_map.len()))?;
            for (locale, path) in locales_map {
                map.serialize_entry(&locale.to_string(),path)?;
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
            let s = HashMap::<String, String>::deserialize(deserializer)?;
            let mut result: HashMap<Locale, PathBuf> = HashMap::new();

            for entry in s {
                result.insert(
                    entry.0.parse().expect("Parsing the locale failed"),
                    PathBuf::from(entry.1),
                );
            }
            Ok(result)
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
