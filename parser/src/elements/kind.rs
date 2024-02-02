//! Contains [`PossibleBlockStart`] to help selecting possible parsers in the main block parser.

use unimarkup_commons::lexer::token::TokenKind;

/// Enum helping to return possible parser functions of elements the following tokens may resolve to.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PossibleBlockStart {
    /// Denotes that the following tokens may resolve to a heading element.
    Heading(usize),
    /// Denotes that the following tokens may resolve to a column block.
    ColumnBlock,
    /// Denotes that the following tokens may resolve to a math block.
    MathBlock,
    /// Denotes that the following tokens may resolve to a render block.
    RenderBlock,
    /// Denotes that the following tokens may resolve to a verbatim block.
    VerbatimBlock,
    /// Denotes that the following tokens may resolve to a table.
    Table,
    /// Denotes that the following tokens may resolve to a bullet list.
    BulletList,
    /// The following tokens may resolve to a numbered list,
    /// but the required `dot` must be checked in the parser function itself.
    Digit,
    /// Denotes that the following tokens may resolve to a quotation block.
    QuotationBlock,
    /// Denotes that the following tokens may resolve to a line block.
    LineBlock,
    /// Denotes that the following tokens may resolve to a media insert.
    MediaInsert,
    /// Denotes that the following tokens may resolve to a render insert.
    RenderInsert,
    /// Denotes that the following tokens may resolve to a verbatim insert.
    VerbatimInsert,
    /// Denotes that the following tokens may resolve to a horizontal line.
    HorizontalLine,
    /// Denotes that the following tokens may resolve to a line break.
    LineBreak,
    /// Denotes that the following tokens may resolve to a block decoration.
    Decoration,
    /// Every token that may not lead to another block element can only lead to a paragraph.
    #[default]
    Paragraph,
    /// Denotes that the following tokens may resolve to a text/field block.
    OpenBracket,
    /// Denotes that the following tokens may resolve to an attribute block.
    OpenBrace,
}

impl From<TokenKind> for PossibleBlockStart {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Star(1) => {
                return PossibleBlockStart::BulletList;
            }
            TokenKind::Star(_) => {
                return PossibleBlockStart::Paragraph;
            }
            TokenKind::Hash(len) => {
                if len <= 6 && len > 0 {
                    return PossibleBlockStart::Heading(len);
                }
            }
            TokenKind::Minus(len) => {
                if len == 1 {
                    return PossibleBlockStart::BulletList;
                } else if len >= 3 {
                    return PossibleBlockStart::HorizontalLine;
                }
            }
            TokenKind::Plus(len) => {
                if len == 1 {
                    return PossibleBlockStart::BulletList;
                } else if len >= 3 {
                    return PossibleBlockStart::Decoration;
                }
            }
            TokenKind::Tick(len) => {
                if len >= 3 {
                    return PossibleBlockStart::VerbatimBlock;
                }
            }
            TokenKind::Underline(_)
            | TokenKind::Caret(_)
            | TokenKind::Overline(_)
            | TokenKind::Pipe(_)
            | TokenKind::Tilde(_)
            | TokenKind::Quote(_)
            | TokenKind::Dollar(_)
            | TokenKind::Colon(_)
            | TokenKind::Dot(_)
            | TokenKind::Ampersand(_)
            | TokenKind::Comma(_)
            | TokenKind::OpenParenthesis
            | TokenKind::CloseParenthesis
            | TokenKind::OpenBracket
            | TokenKind::CloseBracket
            | TokenKind::OpenBrace
            | TokenKind::CloseBrace
            | TokenKind::Whitespace
            | TokenKind::Newline
            | TokenKind::Blankline
            | TokenKind::Eoi
            | TokenKind::EscapedPlain
            | TokenKind::EscapedWhitespace
            | TokenKind::EscapedNewline
            | TokenKind::Plain
            | TokenKind::TerminalPunctuation
            | TokenKind::Comment { .. }
            | TokenKind::ImplicitSubstitution(_)
            | TokenKind::DirectUri
            | TokenKind::Any
            | TokenKind::Space
            | TokenKind::EnclosedBlockEnd
            | TokenKind::PossibleAttributes
            | TokenKind::PossibleDecorator => {}
        }

        PossibleBlockStart::Paragraph
    }
}
