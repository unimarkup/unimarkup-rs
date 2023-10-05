use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::Config;

use super::test_file::{Test, TestOutputs};

static HTML_ID_MATCHER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(<[^\s]+)[^i]*id=(?:"|')[^"']*(?:"|')([^>]*>)"#).unwrap());

pub fn assert_um_spec<C>(test_group: &str, test: &Test, config: Config, compile: C)
where
    C: Fn(&Test, Config) -> TestOutputs,
{
    let outputs = compile(test, config);

    if let Some(expected_html) = &test.outputs.html {
        let expected_html = expected_html.trim();

        let mut act_html_body = outputs
            .html
            .expect("Compiler should render HTML for the given input");

        // Remove generated ID if expected did not enforce it
        if !HTML_ID_MATCHER.is_match(expected_html) {
            act_html_body = HTML_ID_MATCHER
                .replace(&act_html_body, "${1}${2}")
                .to_string();
        }

        assert!(
            act_html_body.contains(expected_html),
            "{}-{}: Actual HTML body did not contain expected html. Actual: '{}'. Expected: '{}'",
            test_group,
            test.name,
            act_html_body,
            expected_html,
        );
    }
}
