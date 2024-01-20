mod csl_files;

use logid::log;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use crate::csl_json::csl_types::{CslData, CslItem};
use crate::html::citeproc::csl_files::{get_locale_string, get_style_string};
use crate::log_id::{CiteError, GeneralWarning};
use rustyscript::{import, json_args, serde_json, ModuleWrapper};
use unimarkup_commons::config::icu_locid::{Locale, locale};
use unimarkup_parser::document::Document;

pub struct CiteprocWrapper {
    module: ModuleWrapper,
}

impl CiteprocWrapper {
    pub fn new() -> Result<CiteprocWrapper, CiteError> {
        match import("render/src/html/citeproc/js/citeproc_adapter.js") {
            Ok(module) => Ok(CiteprocWrapper { module }),
            Err(_err) => Err(CiteError::ModuleImportError),
        }
    }

    #[cfg(test)]
    pub fn new_with_path(path: &str) -> Result<CiteprocWrapper, CiteError> {
        match import(path) {
            Ok(module) => Ok(CiteprocWrapper { module }),
            Err(_err) => Err(CiteError::ModuleImportError),
        }
    }

    pub fn init_processor(&mut self, doc: &Document, for_pagedjs: bool) -> Result<(), CiteError> {
        let csl_data = get_csl_data(&doc.config.preamble.cite.references);
        let style_id = doc
            .config
            .preamble
            .cite
            .style
            .clone()
            .unwrap_or(PathBuf::from(String::from("ieee")));
        let doc_locale = doc
            .config
            .preamble
            .i18n
            .lang
            .clone()
            .unwrap_or(locale!("en-US"));
        let citation_locales = doc.config.preamble.cite.citation_locales.clone();

        let citation_text = serde_json::ser::to_string(&csl_data).unwrap();
        let locale = get_locale_string(doc_locale, citation_locales);
        let style = get_style_string(style_id);

        self.module
            .call::<()>(
                "initProcessor",
                json_args!(citation_text, locale, style, for_pagedjs),
            )
            .map_err(|_| CiteError::ProcessorInitializationError)
    }

    // returns the citation strings to be placed inline in the same order as the citation_ids
    // the CitationItems have to have the same order that they should appear in the output, because this considers
    // disambiguation and short forms of citations if the same entry was cited before
    pub fn get_citation_strings(
        &mut self,
        doc: &Document,
        citation_id_vectors: &[serde_json::Value],
        for_pagedjs: bool,
    ) -> Result<Vec<String>, CiteError> {
        match self.init_processor(doc, for_pagedjs) {
            Ok(_) =>
                self.module
                    .call("getCitationStrings", citation_id_vectors)
                    .map_err(|_| CiteError::CitationError),
            Err(e) => Err(e)
        }
    }

    pub fn get_author_only(&mut self, doc: &Document, cite_id: String) -> Result<String, CiteError> {
        match self.init_processor(doc, false) {
            Ok(_) => self.module.call("getAuthorOnly", json_args!(cite_id))
                .map_err(|_| CiteError::CitationError),
            Err(e) => Err(e)
        }
    }

    pub fn get_footnotes(&mut self) -> Result<String, CiteError> {
        let has_footnotes = self
            .module
            .call("hasFootnotes", json_args!())
            .map_err(|_| CiteError::CheckForFootnotesError)?;
        if has_footnotes {
            self.module
                .call("getFootnotesString", json_args!())
                .map_err(|_| CiteError::GetFootnotesError)
        } else {
            Ok("".to_string())
        }
    }

    pub fn get_bibliography(&mut self) -> Result<String, CiteError> {
        self.module
            .call("getBibliography", json_args!())
            .map_err(|_| CiteError::GetBibliographyError)
    }
}

pub fn get_csl_data(references: &HashSet<PathBuf>) -> CslData {
    let mut citation_items: Vec<CslItem> = vec![];
    for reference in references {
        if let Ok(citation_string) = fs::read_to_string(reference.clone().into_os_string()) {
            match serde_json::from_str::<CslData>(&citation_string) {
                Ok(mut citation_data) => citation_items.append(&mut citation_data.items),
                Err(e) => {
                    log!(
                        GeneralWarning::JSONDeserialization,
                        format!("JSON deserializaion failed with error: '{:?}'", e)
                    );
                }
            }
        } else {
            log!(
                GeneralWarning::FileRead,
                format!("Could not read csl file: '{:?}'", &reference),
            );
        }
    }
    CslData {
        items: citation_items,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use unimarkup_commons::config::icu_locid::locale;

    #[test]
    fn test_no_footnotes() {
        let mut under_test =
            CiteprocWrapper::new_with_path("./src/html/citeproc/js/citeproc_adapter.js").unwrap();
        let mut citation_paths: HashSet<PathBuf> = HashSet::new();
        citation_paths.insert(
            PathBuf::from_str("./src/html/citeproc/test_files/citation_items.csl").unwrap(),
        );
        let doc_locale = locale!("de-DE");
        let mut citation_locales: HashMap<Locale, PathBuf> = HashMap::new();
        citation_locales.insert(
            locale!("de-DE"),
            PathBuf::from_str("./csl_locales/locales-de-DE.xml").unwrap(),
        );
        let style_id = PathBuf::from_str("./csl_styles/apa.csl").unwrap();
        let serde_result = serde_json::to_value(vec![vec!["id-1"], vec!["id-1", "id-2"]]).unwrap();
        let for_pagedjs = false;
        let actual_citations = under_test.get_citation_strings(
            get_csl_data(&citation_paths),
            doc_locale,
            citation_locales,
            style_id,
            &[serde_result],
            for_pagedjs,
        );

        assert!(actual_citations.is_ok(), "A cite error occurred");
        let unwrapped_actual = actual_citations.unwrap();
        assert_eq!(unwrapped_actual.len(), 2);
        assert!(unwrapped_actual[0].starts_with("<a href"));
        assert!(!unwrapped_actual[0].starts_with("<a href=\"#footnote"));
        assert!(unwrapped_actual[1].starts_with("<a href"));
        assert!(!unwrapped_actual[1].starts_with("<a href=\"#footnote"));

        let actual_footnotes = under_test.get_footnotes();
        assert!(actual_footnotes.unwrap().is_empty());

        let actual_bibliography = under_test.get_bibliography().unwrap();
        assert!(!actual_bibliography.is_empty());
        assert!(actual_bibliography.starts_with("<div class=\"csl-bib-body\""));
    }

    #[test]
    fn test_footnotes() {
        let mut under_test =
            CiteprocWrapper::new_with_path("./src/html/citeproc/js/citeproc_adapter.js").unwrap();
        let mut citation_paths: HashSet<PathBuf> = HashSet::new();
        citation_paths.insert(
            PathBuf::from_str("./src/html/citeproc/test_files/citation_items.csl").unwrap(),
        );
        let doc_locale = locale!("de-DE");
        let mut citation_locales: HashMap<Locale, PathBuf> = HashMap::new();
        citation_locales.insert(
            locale!("de-DE"),
            PathBuf::from_str("./csl_locales/locales-de-DE.xml").unwrap(),
        );
        let style_id = PathBuf::from_str("./csl_styles/chicago-fullnote-bibliography.csl").unwrap();
        let serde_result = serde_json::to_value(vec![vec!["id-1"], vec!["id-1", "id-2"]]).unwrap();
        let for_pagedjs = false;
        let actual_citations = under_test.get_citation_strings(
            get_csl_data(&citation_paths),
            doc_locale,
            citation_locales,
            style_id,
            &[serde_result],
            for_pagedjs,
        );

        assert!(actual_citations.is_ok(), "A cite error occurred");
        let unwrapped_actual = actual_citations.unwrap();
        assert_eq!(unwrapped_actual.len(), 2);
        assert!(unwrapped_actual[0].starts_with("<a href=\"#footnote"));
        assert!(unwrapped_actual[1].starts_with("<a href=\"#footnote"));

        let actual_footnotes = under_test.get_footnotes().unwrap();
        assert!(!actual_footnotes.is_empty());
        assert!(actual_footnotes.starts_with("<div"));

        let actual_bibliography = under_test.get_bibliography().unwrap();
        assert!(!actual_bibliography.is_empty());
        assert!(actual_bibliography.starts_with("<div class=\"csl-bib-body\""));
    }

    #[test]
    fn test_for_pagedjs() {
        let mut under_test =
            CiteprocWrapper::new_with_path("./src/html/citeproc/js/citeproc_adapter.js").unwrap();
        let mut citation_paths: HashSet<PathBuf> = HashSet::new();
        citation_paths.insert(
            PathBuf::from_str("./src/html/citeproc/test_files/citation_items.csl").unwrap(),
        );
        let doc_locale = locale!("de-DE");
        let mut citation_locales: HashMap<Locale, PathBuf> = HashMap::new();
        citation_locales.insert(
            locale!("de-DE"),
            PathBuf::from_str("./csl_locales/locales-de-DE.xml").unwrap(),
        );
        let style_id = PathBuf::from_str("./csl_styles/chicago-fullnote-bibliography.csl").unwrap();
        let serde_result = serde_json::to_value(vec![vec!["id-1"], vec!["id-1", "id-2"]]).unwrap();
        let for_pagedjs = true;
        let actual_citations = under_test.get_citation_strings(
            get_csl_data(&citation_paths),
            doc_locale,
            citation_locales,
            style_id,
            &[serde_result],
            for_pagedjs,
        );

        assert!(actual_citations.is_ok(), "A cite error occurred");
        let unwrapped_actual = actual_citations.unwrap();
        assert_eq!(unwrapped_actual.len(), 2);
        assert!(unwrapped_actual[0].starts_with("<span className=\"footnote\""));
        assert!(unwrapped_actual[1].starts_with("<span className=\"footnote\""));

        let actual_footnotes = under_test.get_footnotes().unwrap();
        assert!(actual_footnotes.is_empty());

        let actual_bibliography = under_test.get_bibliography().unwrap();
        assert!(!actual_bibliography.is_empty());
        assert!(actual_bibliography.starts_with("<div class=\"csl-bib-body\""));
    }

    #[test]
    fn test_get_csl_string_two_files() {
        let mut paths = HashSet::new();
        paths.insert(
            PathBuf::from_str("./src/html/citeproc/test_files/citation_items.csl").unwrap(),
        );
        paths.insert(
            PathBuf::from_str("./src/html/citeproc/test_files/citation_items2.csl").unwrap(),
        );
        let actual_object: CslData = get_csl_data(&paths);

        assert_eq!(actual_object.items.len(), 8);
    }
}
