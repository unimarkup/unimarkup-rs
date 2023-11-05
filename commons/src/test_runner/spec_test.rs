use once_cell::sync::Lazy;
use regex::Regex;

use crate::config::Config;

use super::test_file::{Test, TestOutputs};

static HTML_SPACE_REMOVER: Lazy<Regex> = Lazy::new(|| Regex::new(r">\s+<").unwrap());

pub fn assert_um_spec<C>(test_group: &str, test: &Test, config: Config, compile: C)
where
    C: Fn(&Test, Config) -> TestOutputs,
{
    let outputs = compile(test, config);

    if let Some(expected_html) = &test.outputs.html {
        let expected_html = HTML_SPACE_REMOVER.replace_all(expected_html.trim(), "><");

        let act_html = outputs
            .html
            .expect("Compiler should render HTML for the given input");

        assert!(
            act_html.contains(&*expected_html),
            "{}-{}: Actual HTML body did not contain expected html. Actual: '{}'. Expected: '{}'",
            test_group,
            test.name,
            act_html,
            expected_html,
        );
    }
}
