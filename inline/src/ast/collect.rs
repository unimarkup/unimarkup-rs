//! This module provides functionality to create a Unimarkup inline AST out of a given list of tokens.

use crate::tokenizer::{Position, TokenKind, Tokens, Newline};

use super::{Span, NestedInline, InlineKind, FlatInline, substitutions::DirectSubstitution, Inline, FlattenInlineKind};

/// Struct to store partial collected inline tokens.
/// 
/// Needed for nested tokens.
pub(crate) struct InlineSection {
  /// Partially collected inline tokens.
  pub(crate) content: Inline,
  /// End position of the last inline token of the section.
  pub(crate) end: Position,
}

/// Trait to create an inline AST.
pub(crate) trait InlineAst {
  /// Function to create an inline AST from a given input.
  fn collect(self) -> Inline;
}

impl InlineAst for Tokens {
  fn collect(mut self) -> Inline {
    self.reverse(); // needed to use .pop()
    collect_until(&mut self, TokenKind::Eoi).content
  }
}

/// Function to collect inline elements up until a certain token is reached.
/// 
/// Note: The token of kind `token_kind` is the last token of the returned section, if it was found.
/// Otherwise, the given list of tokens is fully emptied.
pub(crate) fn collect_until(tokens: &mut Tokens, token_kind: TokenKind) -> InlineSection {
  let mut inline = Vec::new();
  let mut end: Position = Position::default();
  let mut prev_token_kind: TokenKind = TokenKind::NewLine; // important to start with space or newline for substitutions

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
          content: content.flatten(),
          span: Span { start: token.position, end }
        };
        inline.push(InlineKind::Verbatim(nested));
      },
      TokenKind::Plain => {
        if prev_token_kind.is_space_or_newline() &&
          ((tokens.last().is_some() && tokens.last().unwrap().is_space_or_newline()) || tokens.last().is_none()) {

          token.content = token.content.substitute_arrow().substitute_emoji();
        }

        let flat = FlatInline{ 
          content: token.content,
          span: Span { start: token.position, end }
        };

        if let Some(InlineKind::Plain(plain)) = inline.last_mut() {
          plain.content.push_str(&flat.content);
          plain.span.end = flat.span.end;
        } else {
          inline.push(InlineKind::Plain(flat));
        }        
      },
      TokenKind::EscapedGrapheme => {
        end.column += 1; // add backlash offset

        let flat = FlatInline{ 
          content: token.content,
          span: Span { start: token.position, end }
        };

        if flat.content.is_newline() {
          inline.push(InlineKind::EscapedNewLine(flat));
        } else if flat.content.contains(char::is_whitespace) {
          inline.push(InlineKind::EscapedSpace(flat));
        } else if let Some(InlineKind::Plain(plain_flat)) = inline.last_mut() {
          plain_flat.content.push_str(&flat.content);
          plain_flat.span.end = flat.span.end;
        } else {
          inline.push(InlineKind::Plain(flat));
        }
      },
      TokenKind::NewLine
      | TokenKind::Space => {
        // Newline and space are converted to single ascii whitespace

        let flat = FlatInline{ 
          content: " ".to_string(),
          span: Span { start: token.position, end }
        };

        if let Some(InlineKind::Plain(plain)) = inline.last_mut() {
          plain.content.push_str(&flat.content);
          plain.span.end = flat.span.end;
        } else {
          inline.push(InlineKind::Plain(flat));
        }
      },
      TokenKind::TextGroupOpen => todo!(),
      _ => todo!(),
    }

    prev_token_kind = token.kind;
  }

  InlineSection{ content: inline, end }
}
