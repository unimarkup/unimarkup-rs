use unicode_segmentation::UnicodeSegmentation;

/// CursorPos struct is used to keep track of the cursor position while parsing input.
/// Additionally it is useful as a way to show better error messages. Using CursorPos it
/// is possible to show exact location in the input, where parsing error is recognized.
#[derive(Copy, Clone, Debug)]
pub struct CursorPos {
    /// Index of the line in the given input
    pub line: usize,
    /// index of the symbol in the given line
    pub symbol: usize,
}

pub fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}

pub fn count_symbol_until(
    line: &str,
    symbol: &str,
    until: fn(char) -> bool,
) -> Result<usize, (usize, String)> {
    let mut symbols = line.graphemes(true);

    let count = symbols
        .by_ref()
        .peekable()
        .take_while(|&current_symbol| current_symbol == symbol)
        .count();

    let error_message = "Unexpected symbol found!";

    if let Some(symbol) = line.graphemes(true).nth(count) {
        if symbol.contains(until) {
            Ok(count)
        } else {
            let message = String::from(error_message);

            Err((count, message))
        }
    } else {
        let message = String::from(error_message);

        Err((count, message))
    }
}
