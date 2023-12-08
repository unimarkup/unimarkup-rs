#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CslCitation {
    #[serde(rename = "citationID")]
    pub citation_id: CslCitationCitationId,
    #[serde(
        rename = "citationItems",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub citation_items: Vec<CslCitationCitationItemsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CslCitationProperties>,
    pub schema: CslCitationSchema,
}
impl From<&CslCitation> for CslCitation {
    fn from(value: &CslCitation) -> Self {
        value.clone()
    }
}
impl CslCitation {
    pub fn builder() -> builder::CslCitation {
        builder::CslCitation::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslCitationCitationId {
    Number(f64),
    String(String),
}
impl From<&CslCitationCitationId> for CslCitationCitationId {
    fn from(value: &CslCitationCitationId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslCitationCitationId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslCitationCitationId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslCitationCitationId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslCitationCitationId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslCitationCitationId {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslCitationCitationId {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CslCitationCitationItemsItem {
    #[serde(
        rename = "author-only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub author_only: Option<CslCitationCitationItemsItemAuthorOnly>,
    pub id: CslCitationCitationItemsItemId,
    #[serde(rename = "itemData", default, skip_serializing_if = "Option::is_none")]
    pub item_data: Option<CslItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<CslCitationCitationItemsItemLabel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(
        rename = "suppress-author",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppress_author: Option<CslCitationCitationItemsItemSuppressAuthor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uris: Vec<String>,
}
impl From<&CslCitationCitationItemsItem> for CslCitationCitationItemsItem {
    fn from(value: &CslCitationCitationItemsItem) -> Self {
        value.clone()
    }
}
impl CslCitationCitationItemsItem {
    pub fn builder() -> builder::CslCitationCitationItemsItem {
        builder::CslCitationCitationItemsItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslCitationCitationItemsItemAuthorOnly {
    Boolean(bool),
    Number(f64),
    String(String),
}
impl From<&CslCitationCitationItemsItemAuthorOnly> for CslCitationCitationItemsItemAuthorOnly {
    fn from(value: &CslCitationCitationItemsItemAuthorOnly) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslCitationCitationItemsItemAuthorOnly {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslCitationCitationItemsItemAuthorOnly {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslCitationCitationItemsItemAuthorOnly {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslCitationCitationItemsItemAuthorOnly {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslCitationCitationItemsItemAuthorOnly {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(x) => x.to_string(),
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<bool> for CslCitationCitationItemsItemAuthorOnly {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for CslCitationCitationItemsItemAuthorOnly {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslCitationCitationItemsItemId {
    Number(f64),
    String(String),
}
impl From<&CslCitationCitationItemsItemId> for CslCitationCitationItemsItemId {
    fn from(value: &CslCitationCitationItemsItemId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslCitationCitationItemsItemId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslCitationCitationItemsItemId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslCitationCitationItemsItemId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslCitationCitationItemsItemId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslCitationCitationItemsItemId {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslCitationCitationItemsItemId {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CslCitationCitationItemsItemLabel {
    #[serde(rename = "act")]
    Act,
    #[serde(rename = "appendix")]
    Appendix,
    #[serde(rename = "article-locator")]
    ArticleLocator,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "canon")]
    Canon,
    #[serde(rename = "chapter")]
    Chapter,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "elocation")]
    Elocation,
    #[serde(rename = "equation")]
    Equation,
    #[serde(rename = "figure")]
    Figure,
    #[serde(rename = "folio")]
    Folio,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "paragraph")]
    Paragraph,
    #[serde(rename = "part")]
    Part,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "scene")]
    Scene,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "sub-verbo")]
    SubVerbo,
    #[serde(rename = "supplement")]
    Supplement,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "title-locator")]
    TitleLocator,
    #[serde(rename = "verse")]
    Verse,
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "volume")]
    Volume,
}
impl From<&CslCitationCitationItemsItemLabel> for CslCitationCitationItemsItemLabel {
    fn from(value: &CslCitationCitationItemsItemLabel) -> Self {
        value.clone()
    }
}
impl ToString for CslCitationCitationItemsItemLabel {
    fn to_string(&self) -> String {
        match *self {
            Self::Act => "act".to_string(),
            Self::Appendix => "appendix".to_string(),
            Self::ArticleLocator => "article-locator".to_string(),
            Self::Book => "book".to_string(),
            Self::Canon => "canon".to_string(),
            Self::Chapter => "chapter".to_string(),
            Self::Column => "column".to_string(),
            Self::Elocation => "elocation".to_string(),
            Self::Equation => "equation".to_string(),
            Self::Figure => "figure".to_string(),
            Self::Folio => "folio".to_string(),
            Self::Issue => "issue".to_string(),
            Self::Line => "line".to_string(),
            Self::Note => "note".to_string(),
            Self::Opus => "opus".to_string(),
            Self::Page => "page".to_string(),
            Self::Paragraph => "paragraph".to_string(),
            Self::Part => "part".to_string(),
            Self::Rule => "rule".to_string(),
            Self::Scene => "scene".to_string(),
            Self::Section => "section".to_string(),
            Self::SubVerbo => "sub-verbo".to_string(),
            Self::Supplement => "supplement".to_string(),
            Self::Table => "table".to_string(),
            Self::Timestamp => "timestamp".to_string(),
            Self::TitleLocator => "title-locator".to_string(),
            Self::Verse => "verse".to_string(),
            Self::Version => "version".to_string(),
            Self::Volume => "volume".to_string(),
        }
    }
}
impl std::str::FromStr for CslCitationCitationItemsItemLabel {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "act" => Ok(Self::Act),
            "appendix" => Ok(Self::Appendix),
            "article-locator" => Ok(Self::ArticleLocator),
            "book" => Ok(Self::Book),
            "canon" => Ok(Self::Canon),
            "chapter" => Ok(Self::Chapter),
            "column" => Ok(Self::Column),
            "elocation" => Ok(Self::Elocation),
            "equation" => Ok(Self::Equation),
            "figure" => Ok(Self::Figure),
            "folio" => Ok(Self::Folio),
            "issue" => Ok(Self::Issue),
            "line" => Ok(Self::Line),
            "note" => Ok(Self::Note),
            "opus" => Ok(Self::Opus),
            "page" => Ok(Self::Page),
            "paragraph" => Ok(Self::Paragraph),
            "part" => Ok(Self::Part),
            "rule" => Ok(Self::Rule),
            "scene" => Ok(Self::Scene),
            "section" => Ok(Self::Section),
            "sub-verbo" => Ok(Self::SubVerbo),
            "supplement" => Ok(Self::Supplement),
            "table" => Ok(Self::Table),
            "timestamp" => Ok(Self::Timestamp),
            "title-locator" => Ok(Self::TitleLocator),
            "verse" => Ok(Self::Verse),
            "version" => Ok(Self::Version),
            "volume" => Ok(Self::Volume),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for CslCitationCitationItemsItemLabel {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslCitationCitationItemsItemLabel {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslCitationCitationItemsItemLabel {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslCitationCitationItemsItemSuppressAuthor {
    Boolean(bool),
    Number(f64),
    String(String),
}
impl From<&CslCitationCitationItemsItemSuppressAuthor>
    for CslCitationCitationItemsItemSuppressAuthor
{
    fn from(value: &CslCitationCitationItemsItemSuppressAuthor) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslCitationCitationItemsItemSuppressAuthor {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslCitationCitationItemsItemSuppressAuthor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslCitationCitationItemsItemSuppressAuthor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslCitationCitationItemsItemSuppressAuthor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslCitationCitationItemsItemSuppressAuthor {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(x) => x.to_string(),
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<bool> for CslCitationCitationItemsItemSuppressAuthor {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for CslCitationCitationItemsItemSuppressAuthor {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CslCitationProperties {
    #[serde(rename = "noteIndex", default, skip_serializing_if = "Option::is_none")]
    pub note_index: Option<f64>,
}
impl From<&CslCitationProperties> for CslCitationProperties {
    fn from(value: &CslCitationProperties) -> Self {
        value.clone()
    }
}
impl CslCitationProperties {
    pub fn builder() -> builder::CslCitationProperties {
        builder::CslCitationProperties::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CslCitationSchema {
    #[serde(
        rename = "https://resource.citationstyles.org/schema/latest/input/json/csl-citation.json"
    )]
    HttpsResourceCitationstylesOrgSchemaLatestInputJsonCslCitationJson,
}
impl From<&CslCitationSchema> for CslCitationSchema {
    fn from(value: &CslCitationSchema) -> Self {
        value.clone()
    }
}
impl ToString for CslCitationSchema {
    fn to_string(&self) -> String {
        match *self {
            Self::HttpsResourceCitationstylesOrgSchemaLatestInputJsonCslCitationJson => {
                "https://resource.citationstyles.org/schema/latest/input/json/csl-citation.json"
                    .to_string()
            }
        }
    }
}
impl std::str::FromStr for CslCitationSchema {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "https://resource.citationstyles.org/schema/latest/input/json/csl-citation.json" => {
                Ok(Self::HttpsResourceCitationstylesOrgSchemaLatestInputJsonCslCitationJson)
            }
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for CslCitationSchema {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslCitationSchema {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslCitationSchema {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CslData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CslItem>,
}
impl From<&CslData> for CslData {
    fn from(value: &CslData) -> Self {
        value.clone()
    }
}
impl CslData {
    pub fn builder() -> builder::CslData {
        builder::CslData::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CslItem {
    #[serde(rename = "abstract", default, skip_serializing_if = "Option::is_none")]
    pub abstract_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessed: Option<DateVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annote: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_collection: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_location: Option<String>,
    #[serde(
        rename = "archive-place",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_place: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(
        rename = "available-date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_date: Option<DateVariable>,
    #[serde(
        rename = "call-number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub call_number: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chair: Vec<NameVariable>,
    #[serde(
        rename = "chapter-number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chapter_number: Option<CslItemChapterNumber>,
    #[serde(
        rename = "citation-key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citation_key: Option<String>,
    #[serde(
        rename = "citation-label",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citation_label: Option<String>,
    #[serde(
        rename = "citation-number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citation_number: Option<CslItemCitationNumber>,
    #[serde(
        rename = "collection-editor",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub collection_editor: Vec<NameVariable>,
    #[serde(
        rename = "collection-number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_number: Option<CslItemCollectionNumber>,
    #[serde(
        rename = "collection-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub collection_title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compiler: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub composer: Vec<NameVariable>,
    #[serde(
        rename = "container-author",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub container_author: Vec<NameVariable>,
    #[serde(
        rename = "container-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_title: Option<String>,
    #[serde(
        rename = "container-title-short",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_title_short: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub curator: Vec<NameVariable>,
    #[doc = "Used to store additional information that does not have a designated CSL JSON field. The custom field is preferred over the note field for storing custom data, particularly for storing key-value pairs, as the note field is used for user annotations in annotated bibliography styles."]
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub custom: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub director: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    #[serde(rename = "DOI", default, skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<CslItemEdition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<NameVariable>,
    #[serde(
        rename = "editorial-director",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub editorial_director: Vec<NameVariable>,
    #[doc = "[Deprecated - use 'event-title' instead. Will be removed in 1.1]"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(
        rename = "event-date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_date: Option<DateVariable>,
    #[serde(
        rename = "event-place",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_place: Option<String>,
    #[serde(
        rename = "event-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_title: Option<String>,
    #[serde(
        rename = "executive-producer",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub executive_producer: Vec<NameVariable>,
    #[serde(
        rename = "first-reference-note-number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub first_reference_note_number: Option<CslItemFirstReferenceNoteNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guest: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host: Vec<NameVariable>,
    pub id: CslItemId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub illustrator: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interviewer: Vec<NameVariable>,
    #[serde(rename = "ISBN", default, skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    #[serde(rename = "ISSN", default, skip_serializing_if = "Option::is_none")]
    pub issn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<CslItemIssue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued: Option<DateVariable>,
    #[serde(
        rename = "journalAbbreviation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub journal_abbreviation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<CslItemLocator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub narrator: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<CslItemNumber>,
    #[serde(
        rename = "number-of-pages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_pages: Option<CslItemNumberOfPages>,
    #[serde(
        rename = "number-of-volumes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_volumes: Option<CslItemNumberOfVolumes>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizer: Vec<NameVariable>,
    #[serde(
        rename = "original-author",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub original_author: Vec<NameVariable>,
    #[serde(
        rename = "original-date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_date: Option<DateVariable>,
    #[serde(
        rename = "original-publisher",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_publisher: Option<String>,
    #[serde(
        rename = "original-publisher-place",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_publisher_place: Option<String>,
    #[serde(
        rename = "original-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<CslItemPage>,
    #[serde(
        rename = "page-first",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub page_first: Option<CslItemPageFirst>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub part: Option<CslItemPart>,
    #[serde(
        rename = "part-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub part_title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<NameVariable>,
    #[serde(rename = "PMCID", default, skip_serializing_if = "Option::is_none")]
    pub pmcid: Option<String>,
    #[serde(rename = "PMID", default, skip_serializing_if = "Option::is_none")]
    pub pmid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub printing: Option<CslItemPrinting>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub producer: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(
        rename = "publisher-place",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub publisher_place: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<String>,
    #[serde(
        rename = "reviewed-author",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reviewed_author: Vec<NameVariable>,
    #[serde(
        rename = "reviewed-genre",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reviewed_genre: Option<String>,
    #[serde(
        rename = "reviewed-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reviewed_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    #[serde(
        rename = "script-writer",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub script_writer: Vec<NameVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(
        rename = "series-creator",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub series_creator: Vec<NameVariable>,
    #[serde(
        rename = "shortTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted: Option<DateVariable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplement: Option<CslItemSupplement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "title-short",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub title_short: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub translator: Vec<NameVariable>,
    #[serde(rename = "type")]
    pub type_: CslItemType,
    #[serde(rename = "URL", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<CslItemVolume>,
    #[serde(
        rename = "volume-title",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_title: Option<String>,
    #[serde(
        rename = "volume-title-short",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_title_short: Option<String>,
    #[serde(
        rename = "year-suffix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub year_suffix: Option<String>,
}
impl From<&CslItem> for CslItem {
    fn from(value: &CslItem) -> Self {
        value.clone()
    }
}
impl CslItem {
    pub fn builder() -> builder::CslItem {
        builder::CslItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemChapterNumber {
    Number(f64),
    String(String),
}
impl From<&CslItemChapterNumber> for CslItemChapterNumber {
    fn from(value: &CslItemChapterNumber) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemChapterNumber {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemChapterNumber {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemChapterNumber {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemChapterNumber {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemChapterNumber {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemChapterNumber {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemCitationNumber {
    Number(f64),
    String(String),
}
impl From<&CslItemCitationNumber> for CslItemCitationNumber {
    fn from(value: &CslItemCitationNumber) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemCitationNumber {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemCitationNumber {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemCitationNumber {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemCitationNumber {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemCitationNumber {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemCitationNumber {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemCollectionNumber {
    Number(f64),
    String(String),
}
impl From<&CslItemCollectionNumber> for CslItemCollectionNumber {
    fn from(value: &CslItemCollectionNumber) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemCollectionNumber {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemCollectionNumber {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemCollectionNumber {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemCollectionNumber {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemCollectionNumber {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemCollectionNumber {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemEdition {
    Number(f64),
    String(String),
}
impl From<&CslItemEdition> for CslItemEdition {
    fn from(value: &CslItemEdition) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemEdition {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemEdition {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemEdition {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemEdition {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemEdition {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemEdition {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemFirstReferenceNoteNumber {
    Number(f64),
    String(String),
}
impl From<&CslItemFirstReferenceNoteNumber> for CslItemFirstReferenceNoteNumber {
    fn from(value: &CslItemFirstReferenceNoteNumber) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemFirstReferenceNoteNumber {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemFirstReferenceNoteNumber {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemFirstReferenceNoteNumber {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemFirstReferenceNoteNumber {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemFirstReferenceNoteNumber {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemFirstReferenceNoteNumber {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemId {
    Number(f64),
    String(String),
}
impl From<&CslItemId> for CslItemId {
    fn from(value: &CslItemId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemId {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemId {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemIssue {
    Number(f64),
    String(String),
}
impl From<&CslItemIssue> for CslItemIssue {
    fn from(value: &CslItemIssue) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemIssue {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemIssue {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemIssue {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemIssue {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemIssue {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemIssue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemLocator {
    Number(f64),
    String(String),
}
impl From<&CslItemLocator> for CslItemLocator {
    fn from(value: &CslItemLocator) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemLocator {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemLocator {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemLocator {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemLocator {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemLocator {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemLocator {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemNumber {
    Number(f64),
    String(String),
}
impl From<&CslItemNumber> for CslItemNumber {
    fn from(value: &CslItemNumber) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemNumber {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemNumber {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemNumber {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemNumber {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemNumber {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemNumber {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemNumberOfPages {
    Number(f64),
    String(String),
}
impl From<&CslItemNumberOfPages> for CslItemNumberOfPages {
    fn from(value: &CslItemNumberOfPages) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemNumberOfPages {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemNumberOfPages {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemNumberOfPages {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemNumberOfPages {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemNumberOfPages {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemNumberOfPages {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemNumberOfVolumes {
    Number(f64),
    String(String),
}
impl From<&CslItemNumberOfVolumes> for CslItemNumberOfVolumes {
    fn from(value: &CslItemNumberOfVolumes) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemNumberOfVolumes {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemNumberOfVolumes {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemNumberOfVolumes {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemNumberOfVolumes {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemNumberOfVolumes {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemNumberOfVolumes {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemPage {
    Number(f64),
    String(String),
}
impl From<&CslItemPage> for CslItemPage {
    fn from(value: &CslItemPage) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemPage {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemPage {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemPage {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemPage {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemPage {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemPage {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemPageFirst {
    Number(f64),
    String(String),
}
impl From<&CslItemPageFirst> for CslItemPageFirst {
    fn from(value: &CslItemPageFirst) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemPageFirst {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemPageFirst {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemPageFirst {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemPageFirst {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemPageFirst {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemPageFirst {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemPart {
    Number(f64),
    String(String),
}
impl From<&CslItemPart> for CslItemPart {
    fn from(value: &CslItemPart) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemPart {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemPart {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemPart {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemPart {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemPart {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemPart {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemPrinting {
    Number(f64),
    String(String),
}
impl From<&CslItemPrinting> for CslItemPrinting {
    fn from(value: &CslItemPrinting) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemPrinting {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemPrinting {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemPrinting {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemPrinting {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemPrinting {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemPrinting {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemSupplement {
    Number(f64),
    String(String),
}
impl From<&CslItemSupplement> for CslItemSupplement {
    fn from(value: &CslItemSupplement) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemSupplement {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemSupplement {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemSupplement {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemSupplement {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemSupplement {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemSupplement {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CslItemType {
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "article-journal")]
    ArticleJournal,
    #[serde(rename = "article-magazine")]
    ArticleMagazine,
    #[serde(rename = "article-newspaper")]
    ArticleNewspaper,
    #[serde(rename = "bill")]
    Bill,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "broadcast")]
    Broadcast,
    #[serde(rename = "chapter")]
    Chapter,
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "collection")]
    Collection,
    #[serde(rename = "dataset")]
    Dataset,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "entry")]
    Entry,
    #[serde(rename = "entry-dictionary")]
    EntryDictionary,
    #[serde(rename = "entry-encyclopedia")]
    EntryEncyclopedia,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "figure")]
    Figure,
    #[serde(rename = "graphic")]
    Graphic,
    #[serde(rename = "hearing")]
    Hearing,
    #[serde(rename = "interview")]
    Interview,
    #[serde(rename = "legal_case")]
    LegalCase,
    #[serde(rename = "legislation")]
    Legislation,
    #[serde(rename = "manuscript")]
    Manuscript,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "motion_picture")]
    MotionPicture,
    #[serde(rename = "musical_score")]
    MusicalScore,
    #[serde(rename = "pamphlet")]
    Pamphlet,
    #[serde(rename = "paper-conference")]
    PaperConference,
    #[serde(rename = "patent")]
    Patent,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "periodical")]
    Periodical,
    #[serde(rename = "personal_communication")]
    PersonalCommunication,
    #[serde(rename = "post")]
    Post,
    #[serde(rename = "post-weblog")]
    PostWeblog,
    #[serde(rename = "regulation")]
    Regulation,
    #[serde(rename = "report")]
    Report,
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "review-book")]
    ReviewBook,
    #[serde(rename = "software")]
    Software,
    #[serde(rename = "song")]
    Song,
    #[serde(rename = "speech")]
    Speech,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "thesis")]
    Thesis,
    #[serde(rename = "treaty")]
    Treaty,
    #[serde(rename = "webpage")]
    Webpage,
}
impl From<&CslItemType> for CslItemType {
    fn from(value: &CslItemType) -> Self {
        value.clone()
    }
}
impl ToString for CslItemType {
    fn to_string(&self) -> String {
        match *self {
            Self::Article => "article".to_string(),
            Self::ArticleJournal => "article-journal".to_string(),
            Self::ArticleMagazine => "article-magazine".to_string(),
            Self::ArticleNewspaper => "article-newspaper".to_string(),
            Self::Bill => "bill".to_string(),
            Self::Book => "book".to_string(),
            Self::Broadcast => "broadcast".to_string(),
            Self::Chapter => "chapter".to_string(),
            Self::Classic => "classic".to_string(),
            Self::Collection => "collection".to_string(),
            Self::Dataset => "dataset".to_string(),
            Self::Document => "document".to_string(),
            Self::Entry => "entry".to_string(),
            Self::EntryDictionary => "entry-dictionary".to_string(),
            Self::EntryEncyclopedia => "entry-encyclopedia".to_string(),
            Self::Event => "event".to_string(),
            Self::Figure => "figure".to_string(),
            Self::Graphic => "graphic".to_string(),
            Self::Hearing => "hearing".to_string(),
            Self::Interview => "interview".to_string(),
            Self::LegalCase => "legal_case".to_string(),
            Self::Legislation => "legislation".to_string(),
            Self::Manuscript => "manuscript".to_string(),
            Self::Map => "map".to_string(),
            Self::MotionPicture => "motion_picture".to_string(),
            Self::MusicalScore => "musical_score".to_string(),
            Self::Pamphlet => "pamphlet".to_string(),
            Self::PaperConference => "paper-conference".to_string(),
            Self::Patent => "patent".to_string(),
            Self::Performance => "performance".to_string(),
            Self::Periodical => "periodical".to_string(),
            Self::PersonalCommunication => "personal_communication".to_string(),
            Self::Post => "post".to_string(),
            Self::PostWeblog => "post-weblog".to_string(),
            Self::Regulation => "regulation".to_string(),
            Self::Report => "report".to_string(),
            Self::Review => "review".to_string(),
            Self::ReviewBook => "review-book".to_string(),
            Self::Software => "software".to_string(),
            Self::Song => "song".to_string(),
            Self::Speech => "speech".to_string(),
            Self::Standard => "standard".to_string(),
            Self::Thesis => "thesis".to_string(),
            Self::Treaty => "treaty".to_string(),
            Self::Webpage => "webpage".to_string(),
        }
    }
}
impl std::str::FromStr for CslItemType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "article" => Ok(Self::Article),
            "article-journal" => Ok(Self::ArticleJournal),
            "article-magazine" => Ok(Self::ArticleMagazine),
            "article-newspaper" => Ok(Self::ArticleNewspaper),
            "bill" => Ok(Self::Bill),
            "book" => Ok(Self::Book),
            "broadcast" => Ok(Self::Broadcast),
            "chapter" => Ok(Self::Chapter),
            "classic" => Ok(Self::Classic),
            "collection" => Ok(Self::Collection),
            "dataset" => Ok(Self::Dataset),
            "document" => Ok(Self::Document),
            "entry" => Ok(Self::Entry),
            "entry-dictionary" => Ok(Self::EntryDictionary),
            "entry-encyclopedia" => Ok(Self::EntryEncyclopedia),
            "event" => Ok(Self::Event),
            "figure" => Ok(Self::Figure),
            "graphic" => Ok(Self::Graphic),
            "hearing" => Ok(Self::Hearing),
            "interview" => Ok(Self::Interview),
            "legal_case" => Ok(Self::LegalCase),
            "legislation" => Ok(Self::Legislation),
            "manuscript" => Ok(Self::Manuscript),
            "map" => Ok(Self::Map),
            "motion_picture" => Ok(Self::MotionPicture),
            "musical_score" => Ok(Self::MusicalScore),
            "pamphlet" => Ok(Self::Pamphlet),
            "paper-conference" => Ok(Self::PaperConference),
            "patent" => Ok(Self::Patent),
            "performance" => Ok(Self::Performance),
            "periodical" => Ok(Self::Periodical),
            "personal_communication" => Ok(Self::PersonalCommunication),
            "post" => Ok(Self::Post),
            "post-weblog" => Ok(Self::PostWeblog),
            "regulation" => Ok(Self::Regulation),
            "report" => Ok(Self::Report),
            "review" => Ok(Self::Review),
            "review-book" => Ok(Self::ReviewBook),
            "software" => Ok(Self::Software),
            "song" => Ok(Self::Song),
            "speech" => Ok(Self::Speech),
            "standard" => Ok(Self::Standard),
            "thesis" => Ok(Self::Thesis),
            "treaty" => Ok(Self::Treaty),
            "webpage" => Ok(Self::Webpage),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CslItemVolume {
    Number(f64),
    String(String),
}
impl From<&CslItemVolume> for CslItemVolume {
    fn from(value: &CslItemVolume) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CslItemVolume {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for CslItemVolume {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CslItemVolume {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CslItemVolume {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for CslItemVolume {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for CslItemVolume {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DateVariable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circa: Option<DateVariableCirca>,
    #[serde(rename = "date-parts", default, skip_serializing_if = "Vec::is_empty")]
    pub date_parts: Vec<Vec<DateVariableDatePartsItemItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<DateVariableSeason>,
}
impl From<&DateVariable> for DateVariable {
    fn from(value: &DateVariable) -> Self {
        value.clone()
    }
}
impl DateVariable {
    pub fn builder() -> builder::DateVariable {
        builder::DateVariable::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateVariableCirca {
    Boolean(bool),
    Number(f64),
    String(String),
}
impl From<&DateVariableCirca> for DateVariableCirca {
    fn from(value: &DateVariableCirca) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for DateVariableCirca {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for DateVariableCirca {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DateVariableCirca {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DateVariableCirca {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for DateVariableCirca {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(x) => x.to_string(),
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<bool> for DateVariableCirca {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for DateVariableCirca {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateVariableDatePartsItemItem {
    Number(f64),
    String(String),
}
impl From<&DateVariableDatePartsItemItem> for DateVariableDatePartsItemItem {
    fn from(value: &DateVariableDatePartsItemItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for DateVariableDatePartsItemItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for DateVariableDatePartsItemItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DateVariableDatePartsItemItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DateVariableDatePartsItemItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for DateVariableDatePartsItemItem {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for DateVariableDatePartsItemItem {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateVariableSeason {
    Number(f64),
    String(String),
}
impl From<&DateVariableSeason> for DateVariableSeason {
    fn from(value: &DateVariableSeason) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for DateVariableSeason {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for DateVariableSeason {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DateVariableSeason {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DateVariableSeason {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for DateVariableSeason {
    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<f64> for DateVariableSeason {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameVariable {
    #[serde(
        rename = "comma-suffix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comma_suffix: Option<NameVariableCommaSuffix>,
    #[serde(
        rename = "dropping-particle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dropping_particle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<String>,
    #[serde(
        rename = "non-dropping-particle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub non_dropping_particle: Option<String>,
    #[serde(
        rename = "parse-names",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parse_names: Option<NameVariableParseNames>,
    #[serde(
        rename = "static-ordering",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub static_ordering: Option<NameVariableStaticOrdering>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl From<&NameVariable> for NameVariable {
    fn from(value: &NameVariable) -> Self {
        value.clone()
    }
}
impl NameVariable {
    pub fn builder() -> builder::NameVariable {
        builder::NameVariable::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NameVariableCommaSuffix {
    Boolean(bool),
    Number(f64),
    String(String),
}
impl From<&NameVariableCommaSuffix> for NameVariableCommaSuffix {
    fn from(value: &NameVariableCommaSuffix) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NameVariableCommaSuffix {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for NameVariableCommaSuffix {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NameVariableCommaSuffix {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NameVariableCommaSuffix {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for NameVariableCommaSuffix {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(x) => x.to_string(),
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<bool> for NameVariableCommaSuffix {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for NameVariableCommaSuffix {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NameVariableParseNames {
    Boolean(bool),
    Number(f64),
    String(String),
}
impl From<&NameVariableParseNames> for NameVariableParseNames {
    fn from(value: &NameVariableParseNames) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NameVariableParseNames {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for NameVariableParseNames {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NameVariableParseNames {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NameVariableParseNames {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for NameVariableParseNames {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(x) => x.to_string(),
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<bool> for NameVariableParseNames {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for NameVariableParseNames {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NameVariableStaticOrdering {
    Boolean(bool),
    Number(f64),
    String(String),
}
impl From<&NameVariableStaticOrdering> for NameVariableStaticOrdering {
    fn from(value: &NameVariableStaticOrdering) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NameVariableStaticOrdering {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for NameVariableStaticOrdering {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NameVariableStaticOrdering {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NameVariableStaticOrdering {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for NameVariableStaticOrdering {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(x) => x.to_string(),
            Self::Number(x) => x.to_string(),
            Self::String(x) => x.to_string(),
        }
    }
}
impl From<bool> for NameVariableStaticOrdering {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl From<f64> for NameVariableStaticOrdering {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CslCitation {
        citation_id: Result<super::CslCitationCitationId, String>,
        citation_items: Result<Vec<super::CslCitationCitationItemsItem>, String>,
        properties: Result<Option<super::CslCitationProperties>, String>,
        schema: Result<super::CslCitationSchema, String>,
    }
    impl Default for CslCitation {
        fn default() -> Self {
            Self {
                citation_id: Err("no value supplied for citation_id".to_string()),
                citation_items: Ok(Default::default()),
                properties: Ok(Default::default()),
                schema: Err("no value supplied for schema".to_string()),
            }
        }
    }
    impl CslCitation {
        pub fn citation_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CslCitationCitationId>,
            T::Error: std::fmt::Display,
        {
            self.citation_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for citation_id: {}", e));
            self
        }
        pub fn citation_items<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CslCitationCitationItemsItem>>,
            T::Error: std::fmt::Display,
        {
            self.citation_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for citation_items: {}", e));
            self
        }
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslCitationProperties>>,
            T::Error: std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CslCitationSchema>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CslCitation> for super::CslCitation {
        type Error = String;
        fn try_from(value: CslCitation) -> Result<Self, String> {
            Ok(Self {
                citation_id: value.citation_id?,
                citation_items: value.citation_items?,
                properties: value.properties?,
                schema: value.schema?,
            })
        }
    }
    impl From<super::CslCitation> for CslCitation {
        fn from(value: super::CslCitation) -> Self {
            Self {
                citation_id: Ok(value.citation_id),
                citation_items: Ok(value.citation_items),
                properties: Ok(value.properties),
                schema: Ok(value.schema),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CslCitationCitationItemsItem {
        author_only: Result<Option<super::CslCitationCitationItemsItemAuthorOnly>, String>,
        id: Result<super::CslCitationCitationItemsItemId, String>,
        item_data: Result<Option<super::CslItem>, String>,
        label: Result<Option<super::CslCitationCitationItemsItemLabel>, String>,
        locator: Result<Option<String>, String>,
        prefix: Result<Option<String>, String>,
        suffix: Result<Option<String>, String>,
        suppress_author: Result<Option<super::CslCitationCitationItemsItemSuppressAuthor>, String>,
        uris: Result<Vec<String>, String>,
    }
    impl Default for CslCitationCitationItemsItem {
        fn default() -> Self {
            Self {
                author_only: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                item_data: Ok(Default::default()),
                label: Ok(Default::default()),
                locator: Ok(Default::default()),
                prefix: Ok(Default::default()),
                suffix: Ok(Default::default()),
                suppress_author: Ok(Default::default()),
                uris: Ok(Default::default()),
            }
        }
    }
    impl CslCitationCitationItemsItem {
        pub fn author_only<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslCitationCitationItemsItemAuthorOnly>>,
            T::Error: std::fmt::Display,
        {
            self.author_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for author_only: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CslCitationCitationItemsItemId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn item_data<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItem>>,
            T::Error: std::fmt::Display,
        {
            self.item_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_data: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslCitationCitationItemsItemLabel>>,
            T::Error: std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn locator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.locator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for locator: {}", e));
            self
        }
        pub fn prefix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.prefix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prefix: {}", e));
            self
        }
        pub fn suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.suffix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suffix: {}", e));
            self
        }
        pub fn suppress_author<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslCitationCitationItemsItemSuppressAuthor>>,
            T::Error: std::fmt::Display,
        {
            self.suppress_author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suppress_author: {}", e));
            self
        }
        pub fn uris<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.uris = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uris: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CslCitationCitationItemsItem> for super::CslCitationCitationItemsItem {
        type Error = String;
        fn try_from(value: CslCitationCitationItemsItem) -> Result<Self, String> {
            Ok(Self {
                author_only: value.author_only?,
                id: value.id?,
                item_data: value.item_data?,
                label: value.label?,
                locator: value.locator?,
                prefix: value.prefix?,
                suffix: value.suffix?,
                suppress_author: value.suppress_author?,
                uris: value.uris?,
            })
        }
    }
    impl From<super::CslCitationCitationItemsItem> for CslCitationCitationItemsItem {
        fn from(value: super::CslCitationCitationItemsItem) -> Self {
            Self {
                author_only: Ok(value.author_only),
                id: Ok(value.id),
                item_data: Ok(value.item_data),
                label: Ok(value.label),
                locator: Ok(value.locator),
                prefix: Ok(value.prefix),
                suffix: Ok(value.suffix),
                suppress_author: Ok(value.suppress_author),
                uris: Ok(value.uris),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CslCitationProperties {
        note_index: Result<Option<f64>, String>,
    }
    impl Default for CslCitationProperties {
        fn default() -> Self {
            Self {
                note_index: Ok(Default::default()),
            }
        }
    }
    impl CslCitationProperties {
        pub fn note_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.note_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for note_index: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CslCitationProperties> for super::CslCitationProperties {
        type Error = String;
        fn try_from(value: CslCitationProperties) -> Result<Self, String> {
            Ok(Self {
                note_index: value.note_index?,
            })
        }
    }
    impl From<super::CslCitationProperties> for CslCitationProperties {
        fn from(value: super::CslCitationProperties) -> Self {
            Self {
                note_index: Ok(value.note_index),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CslData {
        items: Result<Vec<super::CslItem>, String>,
    }
    impl Default for CslData {
        fn default() -> Self {
            Self {
                items: Ok(Default::default()),
            }
        }
    }
    impl CslData {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CslItem>>,
            T::Error: std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CslData> for super::CslData {
        type Error = String;
        fn try_from(value: CslData) -> Result<Self, String> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl From<super::CslData> for CslData {
        fn from(value: super::CslData) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CslItem {
        abstract_: Result<Option<String>, String>,
        accessed: Result<Option<super::DateVariable>, String>,
        annote: Result<Option<String>, String>,
        archive: Result<Option<String>, String>,
        archive_collection: Result<Option<String>, String>,
        archive_location: Result<Option<String>, String>,
        archive_place: Result<Option<String>, String>,
        author: Result<Vec<super::NameVariable>, String>,
        authority: Result<Option<String>, String>,
        available_date: Result<Option<super::DateVariable>, String>,
        call_number: Result<Option<String>, String>,
        categories: Result<Vec<String>, String>,
        chair: Result<Vec<super::NameVariable>, String>,
        chapter_number: Result<Option<super::CslItemChapterNumber>, String>,
        citation_key: Result<Option<String>, String>,
        citation_label: Result<Option<String>, String>,
        citation_number: Result<Option<super::CslItemCitationNumber>, String>,
        collection_editor: Result<Vec<super::NameVariable>, String>,
        collection_number: Result<Option<super::CslItemCollectionNumber>, String>,
        collection_title: Result<Option<String>, String>,
        compiler: Result<Vec<super::NameVariable>, String>,
        composer: Result<Vec<super::NameVariable>, String>,
        container_author: Result<Vec<super::NameVariable>, String>,
        container_title: Result<Option<String>, String>,
        container_title_short: Result<Option<String>, String>,
        contributor: Result<Vec<super::NameVariable>, String>,
        curator: Result<Vec<super::NameVariable>, String>,
        custom: Result<serde_json::Map<String, serde_json::Value>, String>,
        dimensions: Result<Option<String>, String>,
        director: Result<Vec<super::NameVariable>, String>,
        division: Result<Option<String>, String>,
        doi: Result<Option<String>, String>,
        edition: Result<Option<super::CslItemEdition>, String>,
        editor: Result<Vec<super::NameVariable>, String>,
        editorial_director: Result<Vec<super::NameVariable>, String>,
        event: Result<Option<String>, String>,
        event_date: Result<Option<super::DateVariable>, String>,
        event_place: Result<Option<String>, String>,
        event_title: Result<Option<String>, String>,
        executive_producer: Result<Vec<super::NameVariable>, String>,
        first_reference_note_number: Result<Option<super::CslItemFirstReferenceNoteNumber>, String>,
        genre: Result<Option<String>, String>,
        guest: Result<Vec<super::NameVariable>, String>,
        host: Result<Vec<super::NameVariable>, String>,
        id: Result<super::CslItemId, String>,
        illustrator: Result<Vec<super::NameVariable>, String>,
        interviewer: Result<Vec<super::NameVariable>, String>,
        isbn: Result<Option<String>, String>,
        issn: Result<Option<String>, String>,
        issue: Result<Option<super::CslItemIssue>, String>,
        issued: Result<Option<super::DateVariable>, String>,
        journal_abbreviation: Result<Option<String>, String>,
        jurisdiction: Result<Option<String>, String>,
        keyword: Result<Option<String>, String>,
        language: Result<Option<String>, String>,
        locator: Result<Option<super::CslItemLocator>, String>,
        medium: Result<Option<String>, String>,
        narrator: Result<Vec<super::NameVariable>, String>,
        note: Result<Option<String>, String>,
        number: Result<Option<super::CslItemNumber>, String>,
        number_of_pages: Result<Option<super::CslItemNumberOfPages>, String>,
        number_of_volumes: Result<Option<super::CslItemNumberOfVolumes>, String>,
        organizer: Result<Vec<super::NameVariable>, String>,
        original_author: Result<Vec<super::NameVariable>, String>,
        original_date: Result<Option<super::DateVariable>, String>,
        original_publisher: Result<Option<String>, String>,
        original_publisher_place: Result<Option<String>, String>,
        original_title: Result<Option<String>, String>,
        page: Result<Option<super::CslItemPage>, String>,
        page_first: Result<Option<super::CslItemPageFirst>, String>,
        part: Result<Option<super::CslItemPart>, String>,
        part_title: Result<Option<String>, String>,
        performer: Result<Vec<super::NameVariable>, String>,
        pmcid: Result<Option<String>, String>,
        pmid: Result<Option<String>, String>,
        printing: Result<Option<super::CslItemPrinting>, String>,
        producer: Result<Vec<super::NameVariable>, String>,
        publisher: Result<Option<String>, String>,
        publisher_place: Result<Option<String>, String>,
        recipient: Result<Vec<super::NameVariable>, String>,
        references: Result<Option<String>, String>,
        reviewed_author: Result<Vec<super::NameVariable>, String>,
        reviewed_genre: Result<Option<String>, String>,
        reviewed_title: Result<Option<String>, String>,
        scale: Result<Option<String>, String>,
        script_writer: Result<Vec<super::NameVariable>, String>,
        section: Result<Option<String>, String>,
        series_creator: Result<Vec<super::NameVariable>, String>,
        short_title: Result<Option<String>, String>,
        source: Result<Option<String>, String>,
        status: Result<Option<String>, String>,
        submitted: Result<Option<super::DateVariable>, String>,
        supplement: Result<Option<super::CslItemSupplement>, String>,
        title: Result<Option<String>, String>,
        title_short: Result<Option<String>, String>,
        translator: Result<Vec<super::NameVariable>, String>,
        type_: Result<super::CslItemType, String>,
        url: Result<Option<String>, String>,
        version: Result<Option<String>, String>,
        volume: Result<Option<super::CslItemVolume>, String>,
        volume_title: Result<Option<String>, String>,
        volume_title_short: Result<Option<String>, String>,
        year_suffix: Result<Option<String>, String>,
    }
    impl Default for CslItem {
        fn default() -> Self {
            Self {
                abstract_: Ok(Default::default()),
                accessed: Ok(Default::default()),
                annote: Ok(Default::default()),
                archive: Ok(Default::default()),
                archive_collection: Ok(Default::default()),
                archive_location: Ok(Default::default()),
                archive_place: Ok(Default::default()),
                author: Ok(Default::default()),
                authority: Ok(Default::default()),
                available_date: Ok(Default::default()),
                call_number: Ok(Default::default()),
                categories: Ok(Default::default()),
                chair: Ok(Default::default()),
                chapter_number: Ok(Default::default()),
                citation_key: Ok(Default::default()),
                citation_label: Ok(Default::default()),
                citation_number: Ok(Default::default()),
                collection_editor: Ok(Default::default()),
                collection_number: Ok(Default::default()),
                collection_title: Ok(Default::default()),
                compiler: Ok(Default::default()),
                composer: Ok(Default::default()),
                container_author: Ok(Default::default()),
                container_title: Ok(Default::default()),
                container_title_short: Ok(Default::default()),
                contributor: Ok(Default::default()),
                curator: Ok(Default::default()),
                custom: Ok(Default::default()),
                dimensions: Ok(Default::default()),
                director: Ok(Default::default()),
                division: Ok(Default::default()),
                doi: Ok(Default::default()),
                edition: Ok(Default::default()),
                editor: Ok(Default::default()),
                editorial_director: Ok(Default::default()),
                event: Ok(Default::default()),
                event_date: Ok(Default::default()),
                event_place: Ok(Default::default()),
                event_title: Ok(Default::default()),
                executive_producer: Ok(Default::default()),
                first_reference_note_number: Ok(Default::default()),
                genre: Ok(Default::default()),
                guest: Ok(Default::default()),
                host: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                illustrator: Ok(Default::default()),
                interviewer: Ok(Default::default()),
                isbn: Ok(Default::default()),
                issn: Ok(Default::default()),
                issue: Ok(Default::default()),
                issued: Ok(Default::default()),
                journal_abbreviation: Ok(Default::default()),
                jurisdiction: Ok(Default::default()),
                keyword: Ok(Default::default()),
                language: Ok(Default::default()),
                locator: Ok(Default::default()),
                medium: Ok(Default::default()),
                narrator: Ok(Default::default()),
                note: Ok(Default::default()),
                number: Ok(Default::default()),
                number_of_pages: Ok(Default::default()),
                number_of_volumes: Ok(Default::default()),
                organizer: Ok(Default::default()),
                original_author: Ok(Default::default()),
                original_date: Ok(Default::default()),
                original_publisher: Ok(Default::default()),
                original_publisher_place: Ok(Default::default()),
                original_title: Ok(Default::default()),
                page: Ok(Default::default()),
                page_first: Ok(Default::default()),
                part: Ok(Default::default()),
                part_title: Ok(Default::default()),
                performer: Ok(Default::default()),
                pmcid: Ok(Default::default()),
                pmid: Ok(Default::default()),
                printing: Ok(Default::default()),
                producer: Ok(Default::default()),
                publisher: Ok(Default::default()),
                publisher_place: Ok(Default::default()),
                recipient: Ok(Default::default()),
                references: Ok(Default::default()),
                reviewed_author: Ok(Default::default()),
                reviewed_genre: Ok(Default::default()),
                reviewed_title: Ok(Default::default()),
                scale: Ok(Default::default()),
                script_writer: Ok(Default::default()),
                section: Ok(Default::default()),
                series_creator: Ok(Default::default()),
                short_title: Ok(Default::default()),
                source: Ok(Default::default()),
                status: Ok(Default::default()),
                submitted: Ok(Default::default()),
                supplement: Ok(Default::default()),
                title: Ok(Default::default()),
                title_short: Ok(Default::default()),
                translator: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                url: Ok(Default::default()),
                version: Ok(Default::default()),
                volume: Ok(Default::default()),
                volume_title: Ok(Default::default()),
                volume_title_short: Ok(Default::default()),
                year_suffix: Ok(Default::default()),
            }
        }
    }
    impl CslItem {
        pub fn abstract_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.abstract_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for abstract_: {}", e));
            self
        }
        pub fn accessed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariable>>,
            T::Error: std::fmt::Display,
        {
            self.accessed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accessed: {}", e));
            self
        }
        pub fn annote<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.annote = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for annote: {}", e));
            self
        }
        pub fn archive<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.archive = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for archive: {}", e));
            self
        }
        pub fn archive_collection<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.archive_collection = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for archive_collection: {}",
                    e
                )
            });
            self
        }
        pub fn archive_location<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.archive_location = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for archive_location: {}",
                    e
                )
            });
            self
        }
        pub fn archive_place<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.archive_place = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for archive_place: {}", e));
            self
        }
        pub fn author<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for author: {}", e));
            self
        }
        pub fn authority<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.authority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authority: {}", e));
            self
        }
        pub fn available_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariable>>,
            T::Error: std::fmt::Display,
        {
            self.available_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for available_date: {}", e));
            self
        }
        pub fn call_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.call_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for call_number: {}", e));
            self
        }
        pub fn categories<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.categories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categories: {}", e));
            self
        }
        pub fn chair<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.chair = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chair: {}", e));
            self
        }
        pub fn chapter_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemChapterNumber>>,
            T::Error: std::fmt::Display,
        {
            self.chapter_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for chapter_number: {}", e));
            self
        }
        pub fn citation_key<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.citation_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for citation_key: {}", e));
            self
        }
        pub fn citation_label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.citation_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for citation_label: {}", e));
            self
        }
        pub fn citation_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemCitationNumber>>,
            T::Error: std::fmt::Display,
        {
            self.citation_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for citation_number: {}", e));
            self
        }
        pub fn collection_editor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.collection_editor = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for collection_editor: {}",
                    e
                )
            });
            self
        }
        pub fn collection_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemCollectionNumber>>,
            T::Error: std::fmt::Display,
        {
            self.collection_number = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for collection_number: {}",
                    e
                )
            });
            self
        }
        pub fn collection_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.collection_title = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for collection_title: {}",
                    e
                )
            });
            self
        }
        pub fn compiler<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.compiler = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for compiler: {}", e));
            self
        }
        pub fn composer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.composer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for composer: {}", e));
            self
        }
        pub fn container_author<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.container_author = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for container_author: {}",
                    e
                )
            });
            self
        }
        pub fn container_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.container_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for container_title: {}", e));
            self
        }
        pub fn container_title_short<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.container_title_short = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for container_title_short: {}",
                    e
                )
            });
            self
        }
        pub fn contributor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.contributor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contributor: {}", e));
            self
        }
        pub fn curator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.curator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for curator: {}", e));
            self
        }
        pub fn custom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.custom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for custom: {}", e));
            self
        }
        pub fn dimensions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.dimensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dimensions: {}", e));
            self
        }
        pub fn director<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.director = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for director: {}", e));
            self
        }
        pub fn division<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.division = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for division: {}", e));
            self
        }
        pub fn doi<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.doi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for doi: {}", e));
            self
        }
        pub fn edition<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemEdition>>,
            T::Error: std::fmt::Display,
        {
            self.edition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for edition: {}", e));
            self
        }
        pub fn editor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.editor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for editor: {}", e));
            self
        }
        pub fn editorial_director<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.editorial_director = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for editorial_director: {}",
                    e
                )
            });
            self
        }
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {}", e));
            self
        }
        pub fn event_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariable>>,
            T::Error: std::fmt::Display,
        {
            self.event_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_date: {}", e));
            self
        }
        pub fn event_place<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.event_place = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_place: {}", e));
            self
        }
        pub fn event_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.event_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_title: {}", e));
            self
        }
        pub fn executive_producer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.executive_producer = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for executive_producer: {}",
                    e
                )
            });
            self
        }
        pub fn first_reference_note_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemFirstReferenceNoteNumber>>,
            T::Error: std::fmt::Display,
        {
            self.first_reference_note_number = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for first_reference_note_number: {}",
                    e
                )
            });
            self
        }
        pub fn genre<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.genre = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for genre: {}", e));
            self
        }
        pub fn guest<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.guest = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for guest: {}", e));
            self
        }
        pub fn host<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.host = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for host: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CslItemId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn illustrator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.illustrator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for illustrator: {}", e));
            self
        }
        pub fn interviewer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.interviewer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interviewer: {}", e));
            self
        }
        pub fn isbn<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.isbn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for isbn: {}", e));
            self
        }
        pub fn issn<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.issn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issn: {}", e));
            self
        }
        pub fn issue<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemIssue>>,
            T::Error: std::fmt::Display,
        {
            self.issue = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issue: {}", e));
            self
        }
        pub fn issued<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariable>>,
            T::Error: std::fmt::Display,
        {
            self.issued = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for issued: {}", e));
            self
        }
        pub fn journal_abbreviation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.journal_abbreviation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for journal_abbreviation: {}",
                    e
                )
            });
            self
        }
        pub fn jurisdiction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.jurisdiction = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jurisdiction: {}", e));
            self
        }
        pub fn keyword<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.keyword = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keyword: {}", e));
            self
        }
        pub fn language<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.language = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for language: {}", e));
            self
        }
        pub fn locator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemLocator>>,
            T::Error: std::fmt::Display,
        {
            self.locator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for locator: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn narrator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.narrator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for narrator: {}", e));
            self
        }
        pub fn note<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.note = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for note: {}", e));
            self
        }
        pub fn number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemNumber>>,
            T::Error: std::fmt::Display,
        {
            self.number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number: {}", e));
            self
        }
        pub fn number_of_pages<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemNumberOfPages>>,
            T::Error: std::fmt::Display,
        {
            self.number_of_pages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for number_of_pages: {}", e));
            self
        }
        pub fn number_of_volumes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemNumberOfVolumes>>,
            T::Error: std::fmt::Display,
        {
            self.number_of_volumes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for number_of_volumes: {}",
                    e
                )
            });
            self
        }
        pub fn organizer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.organizer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organizer: {}", e));
            self
        }
        pub fn original_author<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.original_author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for original_author: {}", e));
            self
        }
        pub fn original_date<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariable>>,
            T::Error: std::fmt::Display,
        {
            self.original_date = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for original_date: {}", e));
            self
        }
        pub fn original_publisher<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.original_publisher = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for original_publisher: {}",
                    e
                )
            });
            self
        }
        pub fn original_publisher_place<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.original_publisher_place = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for original_publisher_place: {}",
                    e
                )
            });
            self
        }
        pub fn original_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.original_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for original_title: {}", e));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemPage>>,
            T::Error: std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {}", e));
            self
        }
        pub fn page_first<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemPageFirst>>,
            T::Error: std::fmt::Display,
        {
            self.page_first = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_first: {}", e));
            self
        }
        pub fn part<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemPart>>,
            T::Error: std::fmt::Display,
        {
            self.part = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part: {}", e));
            self
        }
        pub fn part_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.part_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for part_title: {}", e));
            self
        }
        pub fn performer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.performer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for performer: {}", e));
            self
        }
        pub fn pmcid<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pmcid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pmcid: {}", e));
            self
        }
        pub fn pmid<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pmid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pmid: {}", e));
            self
        }
        pub fn printing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemPrinting>>,
            T::Error: std::fmt::Display,
        {
            self.printing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for printing: {}", e));
            self
        }
        pub fn producer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.producer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for producer: {}", e));
            self
        }
        pub fn publisher<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.publisher = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for publisher: {}", e));
            self
        }
        pub fn publisher_place<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.publisher_place = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for publisher_place: {}", e));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {}", e));
            self
        }
        pub fn references<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.references = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for references: {}", e));
            self
        }
        pub fn reviewed_author<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.reviewed_author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reviewed_author: {}", e));
            self
        }
        pub fn reviewed_genre<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.reviewed_genre = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reviewed_genre: {}", e));
            self
        }
        pub fn reviewed_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.reviewed_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reviewed_title: {}", e));
            self
        }
        pub fn scale<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.scale = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scale: {}", e));
            self
        }
        pub fn script_writer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.script_writer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for script_writer: {}", e));
            self
        }
        pub fn section<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.section = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for section: {}", e));
            self
        }
        pub fn series_creator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.series_creator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for series_creator: {}", e));
            self
        }
        pub fn short_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.short_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for short_title: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn submitted<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariable>>,
            T::Error: std::fmt::Display,
        {
            self.submitted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for submitted: {}", e));
            self
        }
        pub fn supplement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemSupplement>>,
            T::Error: std::fmt::Display,
        {
            self.supplement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supplement: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn title_short<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title_short = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title_short: {}", e));
            self
        }
        pub fn translator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::NameVariable>>,
            T::Error: std::fmt::Display,
        {
            self.translator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for translator: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CslItemType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CslItemVolume>>,
            T::Error: std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volume: {}", e));
            self
        }
        pub fn volume_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.volume_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volume_title: {}", e));
            self
        }
        pub fn volume_title_short<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.volume_title_short = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for volume_title_short: {}",
                    e
                )
            });
            self
        }
        pub fn year_suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.year_suffix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for year_suffix: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CslItem> for super::CslItem {
        type Error = String;
        fn try_from(value: CslItem) -> Result<Self, String> {
            Ok(Self {
                abstract_: value.abstract_?,
                accessed: value.accessed?,
                annote: value.annote?,
                archive: value.archive?,
                archive_collection: value.archive_collection?,
                archive_location: value.archive_location?,
                archive_place: value.archive_place?,
                author: value.author?,
                authority: value.authority?,
                available_date: value.available_date?,
                call_number: value.call_number?,
                categories: value.categories?,
                chair: value.chair?,
                chapter_number: value.chapter_number?,
                citation_key: value.citation_key?,
                citation_label: value.citation_label?,
                citation_number: value.citation_number?,
                collection_editor: value.collection_editor?,
                collection_number: value.collection_number?,
                collection_title: value.collection_title?,
                compiler: value.compiler?,
                composer: value.composer?,
                container_author: value.container_author?,
                container_title: value.container_title?,
                container_title_short: value.container_title_short?,
                contributor: value.contributor?,
                curator: value.curator?,
                custom: value.custom?,
                dimensions: value.dimensions?,
                director: value.director?,
                division: value.division?,
                doi: value.doi?,
                edition: value.edition?,
                editor: value.editor?,
                editorial_director: value.editorial_director?,
                event: value.event?,
                event_date: value.event_date?,
                event_place: value.event_place?,
                event_title: value.event_title?,
                executive_producer: value.executive_producer?,
                first_reference_note_number: value.first_reference_note_number?,
                genre: value.genre?,
                guest: value.guest?,
                host: value.host?,
                id: value.id?,
                illustrator: value.illustrator?,
                interviewer: value.interviewer?,
                isbn: value.isbn?,
                issn: value.issn?,
                issue: value.issue?,
                issued: value.issued?,
                journal_abbreviation: value.journal_abbreviation?,
                jurisdiction: value.jurisdiction?,
                keyword: value.keyword?,
                language: value.language?,
                locator: value.locator?,
                medium: value.medium?,
                narrator: value.narrator?,
                note: value.note?,
                number: value.number?,
                number_of_pages: value.number_of_pages?,
                number_of_volumes: value.number_of_volumes?,
                organizer: value.organizer?,
                original_author: value.original_author?,
                original_date: value.original_date?,
                original_publisher: value.original_publisher?,
                original_publisher_place: value.original_publisher_place?,
                original_title: value.original_title?,
                page: value.page?,
                page_first: value.page_first?,
                part: value.part?,
                part_title: value.part_title?,
                performer: value.performer?,
                pmcid: value.pmcid?,
                pmid: value.pmid?,
                printing: value.printing?,
                producer: value.producer?,
                publisher: value.publisher?,
                publisher_place: value.publisher_place?,
                recipient: value.recipient?,
                references: value.references?,
                reviewed_author: value.reviewed_author?,
                reviewed_genre: value.reviewed_genre?,
                reviewed_title: value.reviewed_title?,
                scale: value.scale?,
                script_writer: value.script_writer?,
                section: value.section?,
                series_creator: value.series_creator?,
                short_title: value.short_title?,
                source: value.source?,
                status: value.status?,
                submitted: value.submitted?,
                supplement: value.supplement?,
                title: value.title?,
                title_short: value.title_short?,
                translator: value.translator?,
                type_: value.type_?,
                url: value.url?,
                version: value.version?,
                volume: value.volume?,
                volume_title: value.volume_title?,
                volume_title_short: value.volume_title_short?,
                year_suffix: value.year_suffix?,
            })
        }
    }
    impl From<super::CslItem> for CslItem {
        fn from(value: super::CslItem) -> Self {
            Self {
                abstract_: Ok(value.abstract_),
                accessed: Ok(value.accessed),
                annote: Ok(value.annote),
                archive: Ok(value.archive),
                archive_collection: Ok(value.archive_collection),
                archive_location: Ok(value.archive_location),
                archive_place: Ok(value.archive_place),
                author: Ok(value.author),
                authority: Ok(value.authority),
                available_date: Ok(value.available_date),
                call_number: Ok(value.call_number),
                categories: Ok(value.categories),
                chair: Ok(value.chair),
                chapter_number: Ok(value.chapter_number),
                citation_key: Ok(value.citation_key),
                citation_label: Ok(value.citation_label),
                citation_number: Ok(value.citation_number),
                collection_editor: Ok(value.collection_editor),
                collection_number: Ok(value.collection_number),
                collection_title: Ok(value.collection_title),
                compiler: Ok(value.compiler),
                composer: Ok(value.composer),
                container_author: Ok(value.container_author),
                container_title: Ok(value.container_title),
                container_title_short: Ok(value.container_title_short),
                contributor: Ok(value.contributor),
                curator: Ok(value.curator),
                custom: Ok(value.custom),
                dimensions: Ok(value.dimensions),
                director: Ok(value.director),
                division: Ok(value.division),
                doi: Ok(value.doi),
                edition: Ok(value.edition),
                editor: Ok(value.editor),
                editorial_director: Ok(value.editorial_director),
                event: Ok(value.event),
                event_date: Ok(value.event_date),
                event_place: Ok(value.event_place),
                event_title: Ok(value.event_title),
                executive_producer: Ok(value.executive_producer),
                first_reference_note_number: Ok(value.first_reference_note_number),
                genre: Ok(value.genre),
                guest: Ok(value.guest),
                host: Ok(value.host),
                id: Ok(value.id),
                illustrator: Ok(value.illustrator),
                interviewer: Ok(value.interviewer),
                isbn: Ok(value.isbn),
                issn: Ok(value.issn),
                issue: Ok(value.issue),
                issued: Ok(value.issued),
                journal_abbreviation: Ok(value.journal_abbreviation),
                jurisdiction: Ok(value.jurisdiction),
                keyword: Ok(value.keyword),
                language: Ok(value.language),
                locator: Ok(value.locator),
                medium: Ok(value.medium),
                narrator: Ok(value.narrator),
                note: Ok(value.note),
                number: Ok(value.number),
                number_of_pages: Ok(value.number_of_pages),
                number_of_volumes: Ok(value.number_of_volumes),
                organizer: Ok(value.organizer),
                original_author: Ok(value.original_author),
                original_date: Ok(value.original_date),
                original_publisher: Ok(value.original_publisher),
                original_publisher_place: Ok(value.original_publisher_place),
                original_title: Ok(value.original_title),
                page: Ok(value.page),
                page_first: Ok(value.page_first),
                part: Ok(value.part),
                part_title: Ok(value.part_title),
                performer: Ok(value.performer),
                pmcid: Ok(value.pmcid),
                pmid: Ok(value.pmid),
                printing: Ok(value.printing),
                producer: Ok(value.producer),
                publisher: Ok(value.publisher),
                publisher_place: Ok(value.publisher_place),
                recipient: Ok(value.recipient),
                references: Ok(value.references),
                reviewed_author: Ok(value.reviewed_author),
                reviewed_genre: Ok(value.reviewed_genre),
                reviewed_title: Ok(value.reviewed_title),
                scale: Ok(value.scale),
                script_writer: Ok(value.script_writer),
                section: Ok(value.section),
                series_creator: Ok(value.series_creator),
                short_title: Ok(value.short_title),
                source: Ok(value.source),
                status: Ok(value.status),
                submitted: Ok(value.submitted),
                supplement: Ok(value.supplement),
                title: Ok(value.title),
                title_short: Ok(value.title_short),
                translator: Ok(value.translator),
                type_: Ok(value.type_),
                url: Ok(value.url),
                version: Ok(value.version),
                volume: Ok(value.volume),
                volume_title: Ok(value.volume_title),
                volume_title_short: Ok(value.volume_title_short),
                year_suffix: Ok(value.year_suffix),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DateVariable {
        circa: Result<Option<super::DateVariableCirca>, String>,
        date_parts: Result<Vec<Vec<super::DateVariableDatePartsItemItem>>, String>,
        literal: Result<Option<String>, String>,
        raw: Result<Option<String>, String>,
        season: Result<Option<super::DateVariableSeason>, String>,
    }
    impl Default for DateVariable {
        fn default() -> Self {
            Self {
                circa: Ok(Default::default()),
                date_parts: Ok(Default::default()),
                literal: Ok(Default::default()),
                raw: Ok(Default::default()),
                season: Ok(Default::default()),
            }
        }
    }
    impl DateVariable {
        pub fn circa<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariableCirca>>,
            T::Error: std::fmt::Display,
        {
            self.circa = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for circa: {}", e));
            self
        }
        pub fn date_parts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<Vec<super::DateVariableDatePartsItemItem>>>,
            T::Error: std::fmt::Display,
        {
            self.date_parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date_parts: {}", e));
            self
        }
        pub fn literal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.literal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for literal: {}", e));
            self
        }
        pub fn raw<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.raw = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for raw: {}", e));
            self
        }
        pub fn season<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateVariableSeason>>,
            T::Error: std::fmt::Display,
        {
            self.season = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for season: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DateVariable> for super::DateVariable {
        type Error = String;
        fn try_from(value: DateVariable) -> Result<Self, String> {
            Ok(Self {
                circa: value.circa?,
                date_parts: value.date_parts?,
                literal: value.literal?,
                raw: value.raw?,
                season: value.season?,
            })
        }
    }
    impl From<super::DateVariable> for DateVariable {
        fn from(value: super::DateVariable) -> Self {
            Self {
                circa: Ok(value.circa),
                date_parts: Ok(value.date_parts),
                literal: Ok(value.literal),
                raw: Ok(value.raw),
                season: Ok(value.season),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NameVariable {
        comma_suffix: Result<Option<super::NameVariableCommaSuffix>, String>,
        dropping_particle: Result<Option<String>, String>,
        family: Result<Option<String>, String>,
        given: Result<Option<String>, String>,
        literal: Result<Option<String>, String>,
        non_dropping_particle: Result<Option<String>, String>,
        parse_names: Result<Option<super::NameVariableParseNames>, String>,
        static_ordering: Result<Option<super::NameVariableStaticOrdering>, String>,
        suffix: Result<Option<String>, String>,
    }
    impl Default for NameVariable {
        fn default() -> Self {
            Self {
                comma_suffix: Ok(Default::default()),
                dropping_particle: Ok(Default::default()),
                family: Ok(Default::default()),
                given: Ok(Default::default()),
                literal: Ok(Default::default()),
                non_dropping_particle: Ok(Default::default()),
                parse_names: Ok(Default::default()),
                static_ordering: Ok(Default::default()),
                suffix: Ok(Default::default()),
            }
        }
    }
    impl NameVariable {
        pub fn comma_suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NameVariableCommaSuffix>>,
            T::Error: std::fmt::Display,
        {
            self.comma_suffix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comma_suffix: {}", e));
            self
        }
        pub fn dropping_particle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.dropping_particle = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for dropping_particle: {}",
                    e
                )
            });
            self
        }
        pub fn family<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.family = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for family: {}", e));
            self
        }
        pub fn given<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.given = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for given: {}", e));
            self
        }
        pub fn literal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.literal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for literal: {}", e));
            self
        }
        pub fn non_dropping_particle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.non_dropping_particle = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for non_dropping_particle: {}",
                    e
                )
            });
            self
        }
        pub fn parse_names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NameVariableParseNames>>,
            T::Error: std::fmt::Display,
        {
            self.parse_names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parse_names: {}", e));
            self
        }
        pub fn static_ordering<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NameVariableStaticOrdering>>,
            T::Error: std::fmt::Display,
        {
            self.static_ordering = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for static_ordering: {}", e));
            self
        }
        pub fn suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.suffix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suffix: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<NameVariable> for super::NameVariable {
        type Error = String;
        fn try_from(value: NameVariable) -> Result<Self, String> {
            Ok(Self {
                comma_suffix: value.comma_suffix?,
                dropping_particle: value.dropping_particle?,
                family: value.family?,
                given: value.given?,
                literal: value.literal?,
                non_dropping_particle: value.non_dropping_particle?,
                parse_names: value.parse_names?,
                static_ordering: value.static_ordering?,
                suffix: value.suffix?,
            })
        }
    }
    impl From<super::NameVariable> for NameVariable {
        fn from(value: super::NameVariable) -> Self {
            Self {
                comma_suffix: Ok(value.comma_suffix),
                dropping_particle: Ok(value.dropping_particle),
                family: Ok(value.family),
                given: Ok(value.given),
                literal: Ok(value.literal),
                non_dropping_particle: Ok(value.non_dropping_particle),
                parse_names: Ok(value.parse_names),
                static_ordering: Ok(value.static_ordering),
                suffix: Ok(value.suffix),
            }
        }
    }
}
