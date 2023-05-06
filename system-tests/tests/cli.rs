#![allow(non_snake_case)]

use std::path::PathBuf;

use clap::Parser;
use shlex::Shlex;
use unimarkup_commons::config::{Config, OutputFormat};

// TODO: many test cases must be updated. Commented out for now

fn get_args(options: &str, um_file: &str) -> Vec<String> {
    let arg_line = format!("unimarkup {} {}", options, um_file);
    Shlex::new(&arg_line).collect()
}

#[test]
#[should_panic]
fn test__config_parse__no_arguments_given() {
    let cfg = Config::try_parse_from(vec![""]);

    assert_eq!(
        cfg.unwrap().input,
        PathBuf::new(),
        "UmFile set without being set over cli"
    );
}

#[test]
fn test__config_parse__only_required_arguments_to_struct() {
    let um_filename = "file.um";
    let cfg: Config = Config::parse_from(get_args("", um_filename));

    assert_eq!(
        cfg.input.to_str().unwrap(),
        um_filename,
        "Unimarkup filename not set correctly"
    );
}

#[test]
fn test__config_parse__out_file_option_set() {
    let um_filename = "file.um";
    let out_file = "out_file";

    let mut args = get_args("", um_filename);
    args.push(out_file.to_string()); // pass out_file as second args index

    let cfg: Config = Config::parse_from(args);

    assert_eq!(
        cfg.preamble.output.file.unwrap(),
        PathBuf::from(out_file),
        "Output file not set correctly"
    );
}

#[test]
fn test__config_parse__single_output_format() {
    let um_filename = "file.um";
    let options = "--output-formats=html";

    let cfg: Config = Config::parse_from(get_args(options, um_filename));

    assert_eq!(
        cfg.preamble.output.formats.into_iter().next().unwrap(),
        OutputFormat::Html,
        "Unimarkup html output format not set correctly"
    );
    assert_eq!(
        cfg.input.to_str().unwrap(),
        um_filename,
        "Unimarkup filename not set correctly"
    );
}

#[test]
fn test__config_parse__multiple_output_formats() {
    let um_filename = "file.um";
    // let options = "--output-formats=html,pdf"; // TODO: support pdf output format
    let options = "--output-formats=html";

    let cfg: Config = Config::parse_from(get_args(options, um_filename));
    let formats: Vec<_> = cfg.preamble.output.formats.into_iter().collect();

    assert_eq!(
        formats[0],
        OutputFormat::Html,
        "Unimarkup html output format not set correctly"
    );
    // assert_eq!(
    //     formats[1],
    //     OutputFormat::Pdf,
    //     "Unimarkup html output format not set correctly"
    // );
    assert!(formats.len() == 2, "Too many Unimarkup output formats set");

    assert_eq!(
        cfg.input.to_str().unwrap(),
        um_filename,
        "Unimarkup filename not set correctly"
    );
}

// #[test]
// fn test__config_parse__insert_path_option_set() {
//     let um_filename = "file.um";
//     let insert_path = "~/images";
//
//     let options = format!("--insert-paths={}", insert_path);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(
//         cfg.insert_paths.unwrap()[0],
//         PathBuf::from(insert_path),
//         "Insert path not set correctly"
//     );
// }

// #[test]
// fn test__config_parse__dot_unimarkup_option_set() {
//     let um_filename = "file.um";
//     let dot_unimarkup = "~/.Unimarkup";
//
//     let options = format!("--dot-unimarkup={}", dot_unimarkup);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(
//         cfg.dot_unimarkup.unwrap(),
//         PathBuf::from(dot_unimarkup),
//         "Dot-Unimarkup path not set correctly"
//     );
// }

// #[test]
// fn test__config_parse__theme_option_set() {
//     let um_filename = "file.um";
//     let theme = "theme_file.um";
//
//     let options = format!("--theme={}", theme);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(
//         cfg.theme.unwrap(),
//         PathBuf::from(theme),
//         "Theme file not set correctly"
//     );
// }

// #[test]
// fn test__config_parse__bad_theme_path() {
//     let um_filename = "file.um";
//     let theme = "not_existing_theme.um";
//
//     let options = format!("--theme={}", theme);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert!(!cfg.theme.unwrap().exists(), "Theme file should not exist");
// }

// #[test]
// fn test__config_parse__flags_option_set() {
//     let um_filename = "file.um";
//     let flag = "test";
//
//     let options = format!("--flags={}", flag);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(cfg.flags.unwrap()[0], flag, "Flag not set correctly");
// }

// #[test]
// fn test__config_parse__enable_elements_option_set() {
//     let um_filename = "file.um";
//     let elements = vec![ElementType::Verbatim, ElementType::DefinitionList];
//
//     let options = format!("--enable-elements={},{}", elements[0], elements[1]);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//     let cfg_elements = cfg.enable_elements.unwrap();
//
//     assert_eq!(
//         cfg_elements[0], elements[0],
//         "Enabled element 1 not set correctly"
//     );
//     assert_eq!(
//         cfg_elements[1], elements[1],
//         "Enabled element 2 not set correctly"
//     );
// }

// #[test]
// fn test__config_parse__disable_elements_option_set() {
//     let um_filename = "file.um";
//     let elements = vec![ElementType::Verbatim, ElementType::DefinitionList];
//
//     let options = format!("--disable-elements={},{}", elements[0], elements[1]);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//     let cfg_elements = cfg.disable_elements.unwrap();
//
//     assert_eq!(
//         cfg_elements[0], elements[0],
//         "Disabled element 1 not set correctly"
//     );
//     assert_eq!(
//         cfg_elements[1], elements[1],
//         "Disabled element 2 not set correctly"
//     );
// }

#[test]
#[should_panic]
fn test__config_parse__references_set_without_required_options() {
    let um_filename = "file.um";

    let args = get_args("--references=test.json", um_filename);

    let cfg = Config::try_parse_from(args);

    assert_eq!(
        cfg.unwrap()
            .preamble
            .cite
            .references
            .into_iter()
            .next()
            .unwrap(),
        PathBuf::new(),
        "References set without required options"
    );
}

#[test]
#[should_panic]
fn test__config_parse__citation_style_set_without_required_options() {
    let um_filename = "file.um";

    let args = get_args("--csl=harvard.csl", um_filename);

    let cfg = Config::try_parse_from(args);

    assert_eq!(
        cfg.unwrap().preamble.cite.style.unwrap(),
        PathBuf::new(),
        "Citation style set without required options"
    );
}

#[test]
fn test__config_parse__reference_options_set() {
    let um_filename = "file.um";
    let csl = "apa.csl";
    let refs = "literature.json";

    let options = format!("--citation-style={} --references={}", csl, refs);
    let args = get_args(&options, um_filename);

    let cfg: Config = Config::parse_from(args);

    assert_eq!(
        cfg.preamble.cite.style.unwrap(),
        PathBuf::from(csl),
        "Citation style file not set correctly"
    );
    assert_eq!(
        cfg.preamble.cite.references.into_iter().next().unwrap(),
        PathBuf::from(refs),
        "References file not set correctly"
    );
}

#[test]
fn test__config_parse__fonts_option_set() {
    let um_filename = "file.um";
    let font = "myFont.ttf";

    let options = format!("--fonts={}", font);
    let args = get_args(&options, um_filename);

    let cfg: Config = Config::parse_from(args);

    assert_eq!(
        cfg.preamble.metadata.fonts.into_iter().next().unwrap(),
        PathBuf::from(font),
        "Font file not set correctly"
    );
}

#[test]
fn test__config_parse__overwrite_out_files_option_set() {
    let um_filename = "file.um";

    let args = get_args("--overwrite-out-files", um_filename);

    let cfg: Config = Config::parse_from(args);

    assert!(
        cfg.preamble.output.overwrite,
        "Overwrite-out-files not set correctly"
    );
}

// #[test]
// fn test__config_parse__clean_option_set() {
//     let um_filename = "file.um";
//
//     let args = get_args("--clean", um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert!(cfg.clean, "Clean not set correctly");
// }

// #[test]
// fn test__config_parse__rebuild_option_set() {
//     let um_filename = "file.um";
//
//     let args = get_args("--rebuild", um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert!(cfg.rebuild, "Rebuild not set correctly");
// }

// #[test]
// #[should_panic]
// fn test__config_parse__replace_preamble_set_without_required_options() {
//     let um_filename = "file.um";
//
//     let args = get_args("--replace-preamble", um_filename);
//
//     let cfg = Config::try_parse_from(args);
//
//     assert!(
//         !cfg.unwrap().replace_preamble,
//         "Replace preamble set without required options"
//     );
// }

// #[test]
// fn test__config_parse__replace_preamble_option_set() {
//     let um_filename = "file.um";
//     let out_format = "pdf";
//
//     let options = format!("--replace-preamble --output-formats={}", out_format);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert!(cfg.replace_preamble, "Replace preamble not set correctly");
// }

// #[test]
// fn test__config_parse__relative_insert_prefix_option_set() {
//     let um_filename = "file.um";
//     let insert_prefix = "subdomain/";
//
//     let options = format!("--relative-insert-prefix={}", insert_prefix);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(
//         cfg.relative_insert_prefix.unwrap(),
//         PathBuf::from(insert_prefix),
//         "Relative insert prefix not set correctly"
//     );
// }

// #[test]
// fn test__config_parse__html_template_option_set() {
//     let um_filename = "file.um";
//     let template = "my_template.html";
//
//     let options = format!("--html-template={}", template);
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(
//         cfg.html_template.unwrap(),
//         PathBuf::from(template),
//         "Html template not set correctly"
//     );
// }

//TODO: update these tests!

// #[test]
// fn test__config_parse__html_mathmode_option_set() {
//     let um_filename = "file.um";
//     let mathmode = unimarkup_commons::config::HtmlMathmode::Embed;
//
//     let options = format!("--html-mathmode={}", mathmode.to_string().to_lowercase());
//     let args = get_args(&options, um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert_eq!(
//         cfg.html_mathmode.unwrap(),
//         mathmode,
//         "Html mathmode not set correctly"
//     );
// }

// #[test]
// fn test__config_parse__html_embed_svg_option_set() {
//     let um_filename = "file.um";
//
//     let args = get_args("--html-embed-svg", um_filename);
//
//     let cfg: Config = Config::parse_from(args);
//
//     assert!(cfg.html_embed_svg, "Html embed svg not set correctly");
// }
