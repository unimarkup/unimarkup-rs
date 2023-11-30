mod csl_files;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use rustyscript::{json_args, import, ModuleWrapper, serde_json};
use crate::csl_json::csl_types::{CslData, CslItem};
use crate::html::citeproc::csl_files::{get_locale_string, get_style_string};

pub struct CiteprocWrapper {
    module: ModuleWrapper,
}

impl CiteprocWrapper {

    pub fn new() -> CiteprocWrapper {
        CiteprocWrapper {
            module: import("render/src/html/citeproc/js/citeproc_adapter.js").expect("Importing the JavaScript module failed"),
        }
    }

    // returns the citation strings to be placed inline in the same order as the citation_ids
    // the CitationItems have to have the same order that they should appear in the output, because this considers
    // disambiguation and short forms of citations if the same entry was cited before
    pub fn get_citation_strings(&mut self, citation_paths: &HashSet<PathBuf>, citation_locales: HashSet<PathBuf>, style_id: PathBuf, citation_id_vectors: &Vec<Vec<String>>, for_pagedjs: bool) -> Vec<String> {
        let citation_text = get_csl_string(citation_paths);
        let locale = get_locale_string(citation_locales);
        let style = get_style_string(style_id);

        self.module.call::<()>("initProcessor", json_args!(citation_text, locale, style)).expect("call of initProcessor failed");


        return self.module.call("getCitationStrings", json_args!(serde_json::to_string(citation_id_vectors).unwrap(), for_pagedjs))
            .expect("call of getCitationStrings failed");
    }

    pub fn get_footnotes(&mut self) -> String {
        if self.module.call("hasFootnotes", json_args!()).expect("call of hasFootnotes failed") {
            return self.module.call("getFootnotesString", json_args!()).expect("call of getFootnotesString failed");
        } else {
            return "".to_string();
        }
    }

    pub fn get_bibliography(&mut self) -> String {
        return self.module.call("getBibliography", json_args!()).expect("call of getBibliography failed");
    }
}

fn get_csl_string(references: &HashSet<PathBuf>)-> String {
    let mut citation_items: Vec<CslItem> = vec![];
    for reference in references {
        let citation_string = fs::read_to_string(reference.clone().into_os_string()).expect("reading a reference failed");
        let mut citation_data: CslData = serde_json::from_str::<CslData>(&citation_string)
            .unwrap();
        citation_items.append(&mut citation_data.items);
    }
    let csl_data = CslData {
        items: citation_items
    };
    return serde_json::ser::to_string_pretty(&csl_data).unwrap();
}
