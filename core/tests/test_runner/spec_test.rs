use once_cell::sync::Lazy;
use regex::Regex;

use super::test_file::Test;

static HTML_ID_MATCHER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(<[^\s]+)[^i]*id=(?:"|')[^"']*(?:"|')([^>]*>)"#).unwrap());

pub fn assert_um_spec(test_group: &str, test: &Test, config: unimarkup_core::config::Config) {
    let document = unimarkup_core::unimarkup::compile(&test.input, config).unwrap();

    if let Some(expected_html) = &test.outputs.html {
        let mut act_html_body = document.html().body;
        // Remove generated ID if expected did not enforce it
        if !HTML_ID_MATCHER.is_match(expected_html) {
            act_html_body = HTML_ID_MATCHER
                .replace(&act_html_body, "${1}${2}")
                .to_string();
        }

        assert_eq!(
            act_html_body.trim(),
            expected_html.trim(),
            "{}-{}: Actual (left) HTML body differs from expected (right)",
            test_group,
            test.name
        );
    }
}

#[macro_export]
macro_rules! run_spec_test {
    ($file_path:literal) => {
        let path = PathBuf::from(file!());
        let mut path: PathBuf = path.strip_prefix("core").unwrap().into();

        path.set_file_name("");
        path.push($file_path);

        let mut sub_path: PathBuf = path
            .components()
            .skip_while(|component| Path::new(component) != Path::new("markup"))
            .skip(1)
            .collect();

        sub_path.set_extension("");

        let input = std::fs::read_to_string(&path).unwrap();
        let test_file: $crate::test_runner::test_file::TestFile =
            serde_yaml::from_str(&input).unwrap();

        for test in &test_file.tests {
            // TODO: preamble?

            $crate::test_runner::spec_test::assert_um_spec(
                &test_file.name,
                test,
                unimarkup_core::config::Config::default(),
            );
        }
    };
}
