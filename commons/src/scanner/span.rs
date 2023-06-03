use super::position::Position;

/// Span used to store information about the space some [`Token`] occupies in Unimarkup document.
///
/// [`Token`]: self::Token
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpanLen {
    pub(crate) len_utf8: usize,
    pub(crate) len_utf16: usize,
    pub(crate) len_grapheme: usize,
}

impl From<usize> for SpanLen {
    fn from(value: usize) -> Self {
        SpanLen {
            len_utf8: value,
            len_utf16: value,
            len_grapheme: value,
        }
    }
}

impl Span {
    /// Returns the start position of this span.
    pub fn start(&self) -> Position {
        self.start
    }

    /// Returns the end position of this span.
    pub fn end(&self) -> Position {
        self.end
    }

    /// Returns the number of UTF8 code points this [`Span`] occupies.
    ///
    /// If [`Span`] occupies multiple lines in original
    /// document length cannot be approximated, because it is unknown how long
    /// each of the lines was. In that case None is returned.
    ///
    /// [`Span`]: self::Span
    pub fn len_utf8(&self) -> Option<usize> {
        if self.start.line != self.end.line {
            // Length cannot be approximated for Spans over multiple lines.
            None
        } else {
            Some((self.end - self.start).col_utf8)
        }
    }

    /// Returns the number of UTF16 code points this [`Span`] occupies.
    ///
    /// If [`Span`] occupies multiple lines in original
    /// document length cannot be approximated, because it is unknown how long
    /// each of the lines was. In that case None is returned.
    ///
    /// [`Span`]: self::Span
    pub fn len_utf16(&self) -> Option<usize> {
        if self.start.line != self.end.line {
            // Length cannot be approximated for Spans over multiple lines.
            None
        } else {
            Some((self.end - self.start).col_utf16)
        }
    }

    /// Returns the number of unicode graphems this [`Span`] occupies.
    ///
    /// If [`Span`] occupies multiple lines in original
    /// document length cannot be approximated, because it is unknown how long
    /// each of the lines was. In that case None is returned.
    ///
    /// [`Span`]: self::Span
    pub fn len_grapheme(&self) -> Option<usize> {
        if self.start.line != self.end.line {
            // Length cannot be approximated for Spans over multiple lines.
            None
        } else {
            Some((self.end - self.start).col_grapheme)
        }
    }

    /// Returns the number of UTF8 code points, UTF16 code points and unicode graphems
    /// this [`Span`] occupies.
    ///
    /// If [`Span`] occupies multiple lines in original
    /// document length cannot be approximated, because it is unknown how long
    /// each of the lines was. In that case None is returned.
    ///
    /// [`Span`]: self::Span
    pub fn len(&self) -> Option<SpanLen> {
        let len_utf8 = self.len_utf8()?;
        let len_utf16 = self.len_utf16()?;
        let len_grapheme = self.len_grapheme()?;

        Some(SpanLen {
            len_utf8,
            len_utf16,
            len_grapheme,
        })
    }
}

impl From<(Position, Position)> for Span {
    fn from((start, end): (Position, Position)) -> Self {
        Self { start, end }
    }
}
