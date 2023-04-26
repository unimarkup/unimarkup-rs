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

/// Macro for spec testing of spec files.
/// Spec tests compare the rendered outputs with the expected ones set in the spec files.
///
/// **Arguments:**
///
/// * *file_path* ... A path to the spec file to test, where the path must be relative to the `tests` directory of your crate (e.g. "spec/markup/blocks/paragraph.yml")
///
/// **Usage:**
///
/// ```ignore
/// run_spec_test!("spec/markup/blocks/paragraph.yml");
/// ```
#[macro_export]
macro_rules! run_spec_test {
    ($file_path:literal) => {
        let test_content = $crate::test_runner::test_file::get_test_content($file_path);

        for test in &test_content.test_file.tests {
            // TODO: preamble?

            $crate::test_runner::spec_test::assert_um_spec(
                &test_content.test_file.name,
                test,
                unimarkup_core::config::Config::default(),
            );
        }
    };
}
