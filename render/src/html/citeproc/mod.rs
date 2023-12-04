mod csl_files;

use logid::log;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use crate::csl_json::csl_types::{CslData, CslItem};
use crate::html::citeproc::csl_files::{get_locale_string, get_style_string};
use crate::log_id::{CiteError, GeneralWarning};
use rustyscript::{import, json_args, serde_json, ModuleWrapper};
use unimarkup_commons::config::icu_locid::Locale;

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

    // returns the citation strings to be placed inline in the same order as the citation_ids
    // the CitationItems have to have the same order that they should appear in the output, because this considers
    // disambiguation and short forms of citations if the same entry was cited before
    pub fn get_citation_strings(
        &mut self,
        citation_paths: &HashSet<PathBuf>,
        doc_locale: Locale,
        citation_locales: HashMap<Locale, PathBuf>,
        style_id: PathBuf,
        citation_id_vectors: &[serde_json::Value],
        for_pagedjs: bool,
    ) -> Result<Vec<String>, CiteError> {
        let citation_text = get_csl_string(citation_paths);
        let locale = get_locale_string(doc_locale, citation_locales);
        let style = get_style_string(style_id);

        self.module
            .call::<()>("initProcessor", json_args!(citation_text, locale, style, for_pagedjs))
            .map_err(|_| CiteError::ProcessorInitializationError)?;

        self.module
            .call(
                "getCitationStrings",
                citation_id_vectors,
            )
            .map_err(|_| CiteError::CitationError)
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

fn get_csl_string(references: &HashSet<PathBuf>) -> String {
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
            let mut citation_data: CslData =
                serde_json::from_str::<CslData>(&citation_string).unwrap();
            citation_items.append(&mut citation_data.items);
        } else {
            log!(
                GeneralWarning::FileRead,
                format!("Could not read csl file: '{:?}'", &reference),
            );
        }
    }
    let csl_data = CslData {
        items: citation_items,
    };
    serde_json::ser::to_string_pretty(&csl_data).unwrap()
}
