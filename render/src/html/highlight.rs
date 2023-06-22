//! Provides access to syntax and theme sets and syntax highlighting in general

use once_cell::sync::Lazy;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

/// Constant to get syntax highlighting for a plain text
pub const PLAIN_SYNTAX: &str = "plain";

/// Static reference to the syntax set containing all supported syntaxes
pub static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

/// Highlight content for the given language.
///
/// **Note:** This only adds CSS classes. To see the highlighted content, set `syntax_highlighting_used = true` in the created `HtmlHead`.
pub fn highlight_content(content: &str, language: &str) -> Option<String> {
    let syntax = get_syntax(language);
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        &SYNTAX_SET,
        ClassStyle::SpacedPrefixed {
            prefix: "highlighted_", // Note: Prefix must be in sync with generated stylesheet
        },
    );
    for line in LinesWithEndings::from(content) {
        html_generator
            .parse_html_for_line_which_includes_newline(line)
            .ok()?;
    }
    let highlighted_html = html_generator.finalize();

    // Note: Replace must be in sync with generated stylesheet
    Some(highlighted_html.replace("highlighted_c++", "highlighted_cpp"))
}

/// Get syntax for given language.
/// Falls back to plain text if language is not found.
fn get_syntax(language: &str) -> &'static SyntaxReference {
    if language.to_lowercase() == PLAIN_SYNTAX {
        return SYNTAX_SET.find_syntax_plain_text();
    }

    match SYNTAX_SET.find_syntax_by_token(language) {
        Some(syntax) => syntax,
        None => SYNTAX_SET.find_syntax_plain_text(),
    }
}
