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
            TokenKind::Tick(_) => todo!(),
            TokenKind::Overline(_) => todo!(),
            TokenKind::Pipe(_) => todo!(),
            TokenKind::Tilde(_) => todo!(),
            TokenKind::Quote(_) => todo!(),
            TokenKind::Dollar(_) => todo!(),
            TokenKind::Colon(_) => todo!(),
            TokenKind::Dot(_) => todo!(),
            TokenKind::OpenParenthesis => todo!(),
            TokenKind::CloseParenthesis => todo!(),
            TokenKind::OpenBracket => todo!(),
            TokenKind::CloseBracket => todo!(),
            TokenKind::OpenBrace => todo!(),
            TokenKind::CloseBrace => todo!(),
            TokenKind::Whitespace => todo!(),
            TokenKind::Newline => todo!(),
            TokenKind::Blankline => todo!(),
            TokenKind::Eoi => todo!(),
            TokenKind::EscapedPlain => todo!(),
            TokenKind::EscapedWhitespace => todo!(),
            TokenKind::EscapedNewline => todo!(),
            TokenKind::Plain => todo!(),
            TokenKind::TerminalPunctuation => todo!(),
            TokenKind::Comment { implicit_close } => todo!(),
            TokenKind::ImplicitSubstitution(_) => todo!(),
            TokenKind::DirectUri => todo!(),
            TokenKind::Any => todo!(),
            TokenKind::Space => todo!(),
            TokenKind::PossibleAttributes => todo!(),
            TokenKind::PossibleDecorator => todo!(),
        }

        PossibleBlockStart::Paragraph
    }
}
