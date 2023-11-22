use std::path::PathBuf;

use rustyscript::{json_args, import, ModuleWrapper, serde_json};

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
    // TODO: use locale and style_id
    pub fn get_citation_strings(&mut self, citation_text: String, locale: PathBuf, style_id: PathBuf, citation_id_vectors: &Vec<Vec<String>>, for_pagedjs: bool) -> Vec<String> {
        const LOCALE: &str = include_str!("./files/locales-de-DE.xml");
        const STYLE: &str = include_str!("./files/chicago-fullnote-bibliography.csl");

        self.module.call::<()>("initProcessor", json_args!(citation_text, LOCALE, STYLE)).expect("call of initProcessor failed");


        return self.module.call("getCitationStrings", json_args!(serde_json::to_string(citation_id_vectors).unwrap(), for_pagedjs))
            .expect("call of getCitationStrings failed");
    }

}
