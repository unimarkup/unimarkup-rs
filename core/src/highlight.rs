//! Provides access to syntax and theme sets and syntax highlighting in general

use lazy_static::lazy_static;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

/// The default theme that is used for highlighting
pub const DEFAULT_THEME: &str = "Solarized (dark)";

/// Constant to get syntax highlighting for a plain text
pub const PLAIN_SYNTAX: &str = "plain";

lazy_static! {
    /// Static reference to the syntax set containing all supported syntaxes
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    /// Static reference to the theme set containing all supported themes
    pub static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

/// This function highlights given lines according to set language and theme,
/// returning a standalone HTML string with surrounding `pre` tags.
///
/// If the language is not found in the available syntax set, the first line is analysed.
/// If this also leads to no match, the content is highlighted as plain text.
///
/// If the theme is not supported, a fallback theme is used.
///
/// # Arguments
///
/// * `content` - Content that is being highlighted
/// * `language` - The language to use for highlighting
/// * `theme` - The theme to use for highlighting
///
/// Returns HTML with the highlighted content.
pub fn highlight_html_lines(content: &str, language: &str, theme: &str) -> String {
    let syntax = get_syntax(content.lines().next().unwrap(), language);
    syntect::html::highlighted_html_for_string(content, &SYNTAX_SET, syntax, get_theme(theme))
}

/// This function highlights a single line according to set language and theme to HTML.
///
/// If the language is not found in the available syntax set, the line is analysed.
/// If this also leads to no match, the content is highlighted as plain text.
///
/// If the theme is not supported, a fallback theme is used.
///
/// # Arguments
///
/// * `content` - Content that is being highlighted. Must NOT contain a newline!
/// * `language` - The language to use for highlighting
/// * `theme` - The theme to use for highlighting
///
/// Returns HTML with the highlighted content.
pub fn highlight_single_html_line(one_line: &str, language: &str, theme: &str) -> String {
    let syntax = get_syntax(one_line, language);
    let mut h = HighlightLines::new(syntax, get_theme(theme));
    let regions = h.highlight(one_line, &SYNTAX_SET);
    syntect::html::styled_line_to_highlighted_html(
        &regions[..],
        syntect::html::IncludeBackground::No,
    )
}

/// Get syntax for given language or try to identify syntax by given line.
/// Falls back to plain text if neither matches a syntax.
fn get_syntax(first_line: &str, language: &str) -> &'static SyntaxReference {
    if language.to_lowercase() == PLAIN_SYNTAX {
        return SYNTAX_SET.find_syntax_plain_text();
    }

    match SYNTAX_SET.find_syntax_by_name(language) {
        Some(syntax) => syntax,
        None => match SYNTAX_SET.find_syntax_by_first_line(first_line) {
            Some(syntax) => syntax,
            None => SYNTAX_SET.find_syntax_plain_text(),
        },
    }
}

/// Get theme or fallback, if theme is not found
fn get_theme(theme: &str) -> &'static Theme {
    match THEME_SET.themes.get(theme) {
        Some(theme) => theme,
        None => &THEME_SET.themes[DEFAULT_THEME],
    }
}
