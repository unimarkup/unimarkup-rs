use crate::{
    lexer::{token::Token, token_kind::TokenKind, TokenStream},
    span::Span,
};

pub mod block;

use block::{
    heading::{Heading, HeadingLevel},
    paragraph::Paragraph,
    Block,
};
use ribbon::{Enroll, Ribbon, Tape};

pub struct Parser<'input> {
    /// Iterator that returns tokens found in the Unimarkup input.
    tokens: Tape<TokenStream<'input>>,
}

pub fn parse(input: &str) -> Parser<'_> {
    Parser {
        tokens: TokenStream::tokenize(input).tape(),
    }
}

impl Iterator for Parser<'_> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token = self.tokens.next()?;

            // make sure we can peek next token.
            self.tokens.expand();

            match token.kind {
                TokenKind::Hash(count) => {
                    self.tokens.expand();

                    let is_next_whitespace = self
                        .tokens
                        .peek_front()
                        .map(|token| token.kind == TokenKind::Whitespace)
                        .unwrap_or(false);

                    match HeadingLevel::try_from(count) {
                        Ok(level) if is_next_whitespace => {
                            return self.parse_heading(level).map(Block::Heading);
                        }
                        _ => return self.parse_paragraph(Some(token)),
                    }
                }

                TokenKind::Blankline => {
                    continue;
                }

                _other => return self.parse_paragraph(Some(token)),
                // TokenKind::Star(_) => todo!(),
                // TokenKind::Minus(_) => todo!(),
                // TokenKind::Plus(_) => todo!(),
                // TokenKind::Underline(_) => todo!(),
                // TokenKind::Caret(_) => todo!(),
                // TokenKind::Tick(_) => todo!(),
                // TokenKind::Pipe(_) => todo!(),
                // TokenKind::Tilde(_) => todo!(),
                // TokenKind::Quote(_) => todo!(),
                // TokenKind::Dollar(_) => todo!(),
                // TokenKind::Colon(_) => todo!(),
                // TokenKind::Dot(_) => todo!(),
                // TokenKind::Ampersand(_) => todo!(),
                // TokenKind::Comma(_) => todo!(),
                // TokenKind::OpenParenthesis => todo!(),
                // TokenKind::CloseParenthesis => todo!(),
                // TokenKind::OpenBracket => todo!(),
                // TokenKind::CloseBracket => todo!(),
                // TokenKind::OpenBrace => todo!(),
                // TokenKind::CloseBrace => todo!(),
                // TokenKind::Whitespace => todo!(),
                // TokenKind::Newline => todo!(),
                // TokenKind::Blankline => todo!(),
                // TokenKind::Eoi => todo!(),
                // TokenKind::Indentation(_) => todo!(),
                // TokenKind::EscapedPlain => todo!(),
                // TokenKind::EscapedWhitespace => todo!(),
                // TokenKind::EscapedNewline => todo!(),
                // TokenKind::Plain => todo!(),
                // TokenKind::TerminalPunctuation => todo!(),
                // TokenKind::Comment { implicit_close } => todo!(),
                // TokenKind::DirectUri => todo!(),
                // TokenKind::Any => todo!(),
                // TokenKind::Space => todo!(),
                // TokenKind::EnclosedBlockEnd => todo!(),
                // TokenKind::PossibleAttributes => todo!(),
                // TokenKind::PossibleDecorator => todo!(),
            }
        }
    }
}

impl Parser<'_> {
    fn parse_heading(&mut self, level: HeadingLevel) -> Option<Heading> {
        let expected_indentation = (u8::from(level) + 1) as u32;

        self.tokens.expand_while(|token| match token.kind {
            TokenKind::Indentation(indent_level) => indent_level == expected_indentation,
            TokenKind::Blankline => false,
            _other => true,
        });

        let mut content = String::with_capacity(self.tokens.len());
        let mut span: Option<Span> = None;

        let mut is_start_of_line = true;

        while let Some(token) = self.tokens.pop_front() {
            if let Some(span) = span.as_mut() {
                span.len += token.span.len;
            } else {
                span = Some(token.span);
            }

            match token.kind {
                TokenKind::Whitespace | TokenKind::Space | TokenKind::Indentation(_)
                    if is_start_of_line =>
                {
                    continue;
                }
                _ => content += token.as_input_str(),
            }

            is_start_of_line = matches!(token.kind, TokenKind::Newline | TokenKind::Blankline)
        }

        let span = span?;

        Some(Heading {
            id: String::from("placeholder-id"),
            level,
            content: vec![content],
            attributes: None,
            span,
        })
    }

    fn parse_paragraph(&mut self, first_token: Option<Token<'_>>) -> Option<Block> {
        self.tokens
            .expand_while(|token| token.kind != TokenKind::Blankline);

        let tape_len = self.tokens.len();
        let tape_iter = std::iter::from_fn(|| self.tokens.pop_front());

        let mut content = String::with_capacity(tape_len);
        let mut span: Option<Span> = None;

        for token in std::iter::once(first_token).flatten().chain(tape_iter) {
            if let Some(span) = &mut span {
                span.len += token.span.len;
            } else {
                span = Some(token.span);
            }

            content += token.as_input_str();
        }

        let span = span?;

        Some(Block::Paragraph(Paragraph {
            content: vec![content],
            span,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::block::{heading::HeadingLevel, Block};

    use super::parse;

    #[test]
    fn parse_heading() {
        let input = "## hello there!";

        let heading_block = parse(input)
            .next()
            .expect("Should correctly parse heading!");

        let Block::Heading(heading) = heading_block else {
            panic!("Should correctly parse heading.");
        };

        assert_eq!(heading.level, HeadingLevel::Level2);
    }

    #[test]
    fn invalid_heading() {
        let input = "##hello there!";

        let block = parse(input)
            .next()
            .expect("Should correctly parse heading!");

        let Block::Paragraph(_paragraph) = block else {
            panic!("Should correctly parse heading.");
        };
    }
}
