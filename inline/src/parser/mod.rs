use crate::{inlines::{InlineKind, NestedInline, FlatInline, UnfoldInlineKind}, error::InlineError, Position, Span};

use self::{tokens::{TokenKind, Tokens, Token}, tokenizer::Tokenizer, substitutions::DirectSubstitution};

mod substitutions;
mod tokens;
mod tokenizer;

pub type Inline = Vec<InlineKind>;


pub fn parse(content: &str) -> Result<Inline, InlineError> {
  Ok(content.tokenize()?.collect())
}


trait Parser {
  fn collect(self) -> Inline;
}

impl Parser for Tokens {
  fn collect(mut self) -> Inline {
    self.reverse(); // needed to use .pop()
    collect_until(&mut self, TokenKind::Eoi).content
  }
}

struct InlineSection {
  content: Inline,
  end: Position,
}

fn collect_until(tokens: &mut Tokens, token_kind: TokenKind) -> InlineSection {
  let mut inline = Vec::new();
  let mut end: Position = Position::default();
  let mut prev_token_kind: TokenKind = TokenKind::Eoi;

  while let Some(mut token) = tokens.pop() {
    end = Position{ line: token.position.line, column: token.position.column + token.length() - 1 }; // -1 to use last grapheme as end position
    
    if token.kind == token_kind {
      return InlineSection{ content: inline, end };
    }

    match token.kind {
      TokenKind::BoldOpen => {
        let InlineSection { content, end } = collect_until(tokens, TokenKind::BoldClose);
        let nested = NestedInline{ 
          content,
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::Bold(nested));
      },
      TokenKind::ItalicOpen => {
        let InlineSection { content, end } = collect_until(tokens, TokenKind::ItalicClose);
        let nested = NestedInline{ 
          content,
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::Italic(nested));
      },
      TokenKind::BoldItalicOpen => {
        let InlineSection { content, end } = collect_until(tokens, TokenKind::BoldItalicClose);
        let nested = NestedInline{ 
          content,
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::BoldItalic(nested));
      },
      TokenKind::VerbatimOpen => {
        let InlineSection { content, end } = collect_until(tokens, TokenKind::VerbatimClose);
        let nested = FlatInline{ 
          content: content.to_string(),
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::Verbatim(nested));
      },
      TokenKind::Plain => {
        if prev_token_kind.is_space_or_newline() && tokens.last().is_some() && tokens.last().unwrap().is_space_or_newline() {
          token.content = token.content.substitute_arrow().substitute_emoji();
        }

        if let Some(InlineKind::Plain(flat)) = inline.last_mut() {
          flat.content.push_str(&token.content);
          flat.span.end = end;
          continue;
        }
        
        let flat = FlatInline{ 
          content: token.content,
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::Plain(flat));
      },
      TokenKind::EscapedChar => todo!(),
      TokenKind::NewLine
      | TokenKind::Space => {
        // Newline and space are converted to single ascii whitespace

        if let Some(InlineKind::Plain(flat)) = inline.last_mut() {
          flat.content.push(' ');
          flat.span.end = end;
          continue;
        }
        
        let flat = FlatInline{ 
          content: " ".to_string(),
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::Plain(flat));
      },
      TokenKind::TextGroupOpen => todo!(),
      _ => todo!(),
    }

    prev_token_kind = token.kind;
  }

  InlineSection{ content: inline, end }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
  use super::*;

  pub const EXPECTED_MSG: &str = "actual(left) != expected(right)";

  #[test]
  pub fn test_parser__plain_before_italic() {
    let input = "plain text *italic*";
    let expected = [
      InlineKind::Plain(FlatInline{ 
        content: "plain text ".to_string(),
        span: Span {
          start: Position{
            line: 0,
            column: 0
          },
          end: Position{
            line: 0,
            column: 10
          }
        }
      }),
      InlineKind::Italic(NestedInline{
        content: vec![InlineKind::Plain(FlatInline{
          content: "italic".to_string(),
          span: Span {
            start: Position{
              line: 0,
              column: 12
            },
            end: Position{
              line: 0,
              column: 17
            }
          }
        })],
        span: Span {
          start: Position{
            line: 0,
            column: 11
          },
          end: Position{
            line: 0,
            column: 18
          }
        }
      })
    ];

    let actual = parse(input).unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

}
