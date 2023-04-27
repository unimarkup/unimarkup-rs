use std::{path::PathBuf, collections::HashMap};



pub struct Config {
  pub input: PathBuf,
  pub output: Output,
  pub metadata: Metadata,
  pub cite: Citedata,
  pub render: RenderConfig,
  pub merging: MergingConfig,
  pub i18n: I18n,
}

pub struct I18n {
  pub lang: String,
  pub langs: Vec<String>,
}


pub struct MergingConfig {
  pub ignore_preamble: bool,
}

pub struct RenderConfig {
  /// K = element/attribute name, V = `true` to ignore
  pub ignore: HashMap<String, bool>, 
  pub parameter: Vec<String>,
  pub keep_comments: bool,
  pub non_strict: bool,
}


// TODO: Instead of PathBufs, file contents should be parsed on deserialization.
// This makes it easier to access the parsed data without creating another config struct.
// It also makes compiling faster for bad inputs, since it would break before parsing starts.
pub struct Citedata {
  pub style: PathBuf,
  pub references: Vec<PathBuf>,
}

pub struct Metadata {
  pub title: String,
  pub authors: Vec<String>,
  pub description: String,
  pub base: PathBuf,
  pub fonts: Vec<PathBuf>,
}


pub struct Output {
  pub file: PathBuf,
  pub formats: Vec<OutputFormat>,
  pub format_specific: OutputFormatSpecific,
  /// `true` overwrites existing output files
  pub overwrite: bool,
}

pub enum OutputFormat {
  Html,
}

pub struct OutputFormatSpecific {
  pub html: HtmlSpecific,
}

pub struct HtmlSpecific {
  pub favicon: PathBuf,
  pub keywords: Vec<String>,
}

