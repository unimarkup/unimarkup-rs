use std::str::FromStr;
use log::warn;

use clap::{App, ArgMatches, Values, crate_version, load_yaml};
use crate::config::{Config, HtmlMathmode, OutputConfig, OutputFormat, UmBlockElements};

pub fn get_config_from_cli() -> Config {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let mut cfg = Config {
      um_file : matches.value_of("um_file").unwrap().to_string(),
      out_file : matches.value_of("out_file").unwrap_or_default().to_string(),
      insert_paths : get_strings(matches.values_of("insert_paths")),
      dot_unimarkup: matches.value_of("dot_unimarkup").unwrap_or_default().to_string(),
      theme: matches.value_of("theme").unwrap_or_default().to_string(),
      flags: get_strings(matches.values_of("flags")),
      enable_elements: get_elements(matches.values_of("enable_elements")),
      disable_elements: get_elements(matches.values_of("disable_elements")),
      citation_style: matches.value_of("citation_style").unwrap_or_default().to_string(),
      references: get_strings(matches.values_of("references")),
      fonts: get_strings(matches.values_of("fonts")),
      overwrite_existing: matches.is_present("overwrite_existing"),
      clean: matches.is_present("clean"),
      rebuild: matches.is_present("rebuild"),
      replace_preamble: matches.is_present("replace_preamble"),
      outputs: Vec::new(),
    };

    cfg.outputs = get_config_options(matches, cfg.clone());
    cfg
}

fn get_config_options(args: ArgMatches, config: Config) -> Vec<OutputConfig> {
  let mut out_config = Vec::new();

  if let Some(opts) = args.values_of("options") {
    for option in opts {
      let opt_format = OutputFormat::from_str(option);
      match opt_format {
        Ok(fmt) => {
          let opt = OutputConfig {
            out_file: config.out_file.clone(),
            out_type: fmt,
            theme: config.theme.clone(),
            flags: config.flags.clone(),
            relative_insert_prefix: args.value_of("relative_insert_prefix").unwrap_or_default().to_string(),
            html_template: args.value_of("html_template").unwrap_or("").to_string(),
            html_mathmode: HtmlMathmode::from_str(args.value_of("html_mathmode").unwrap_or("svg")).unwrap_or(HtmlMathmode::Svg),
            html_embed_svg: args.is_present("html_embed_svg"),
          };
          out_config.push(opt);
        },
        Err(_) => {
          warn!("Given output format {} is not supported! See `--help` for usage.", option);
        }
      }
      
    }
  } 
  out_config
}

fn get_strings(values: Option<Values>) -> Vec<String> {
  let mut paths = Vec::new();
  if let Some(vals) = values {
    for val in vals {
      paths.push(val.to_string());
    }
  }
  paths
}

fn get_elements(values: Option<Values>) -> Vec<UmBlockElements> {
  let mut block_elements = Vec::new();

  if let Some(vals) = values {
    for val in vals {
      let element_res = UmBlockElements::from_str(val);
      match element_res {
        Ok(element) => {   
          block_elements.push(element);
        },
        Err(_) => {
          warn!("Given element {} is not a known block element! See `--help` for usage.", val);
        }
      }
    }
  } 
  block_elements
}
