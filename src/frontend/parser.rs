/// CursorPos struct is used to keep track of the cursor position while parsing input.
/// Additionally it is useful as a way to show better error messages. Using CursorPos it
/// is possible to show exact location in the input, where parsing error is recognized.
#[derive(Copy, Clone)]
pub struct CursorPos {
    /// Index of the line in the given input
    pub line: usize,
    /// index of the symbol in the given line
    pub symbol: usize,
}
