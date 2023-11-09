use unimarkup_commons::lexer::token::TokenKind;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PossibleBlockStart {
    Heading(usize),
    ColumnBlock,
    MathBlock,
    RenderBlock,
    VerbatimBlock,
    Table,
    BulletList,
    /// Might lead to valid numbered list
    Digit,
    QuotationBlock,
    LineBlock,
    MediaInsert,
    RenderInsert,
    VerbatimInsert,
    HorizontalLine,
    LineBreak,
    Decoration,
    #[default]
    Paragraph,
    /// Might lead to valid text/field block
    OpenBracket,
    /// Might lead to valid attribute block
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
            TokenKind::Underline(_) => todo!(),
            TokenKind::Caret(_) => todo!(),
            TokenKind::Tick(len) => {
                if len >= 3 {
                    return PossibleBlockStart::VerbatimBlock;
                }
            }
            TokenKind::Overline(_)
            | TokenKind::Pipe(_)
            | TokenKind::Tilde(_)
            | TokenKind::Quote(_)
            | TokenKind::Dollar(_)
            | TokenKind::Colon(_)
            | TokenKind::Dot(_)
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
