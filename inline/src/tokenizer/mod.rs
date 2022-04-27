//! This module provides functionality to tokenize a given &str input.
//! The resulting list of tokens is a flat tokenized representation.
//! 
//! e.g. `*text*` --> `[ItalicOpen][Plain][ItalicClose]`

use std::{collections::{HashMap, hash_map::Entry::Vacant}, cmp::min};

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

mod tokens;
pub use tokens::*;

use crate::error::InlineError;

/// Struct to link to the grapheme position of a token in the given input.
#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Position {
  /// Line number in the given input.
  pub line: usize,
  /// Column in the given input.
  pub column: usize,
}

/// Trait to convert a given input into a list of tokens.
pub trait Tokenizer {
  /// Takes an input and converts it into a list of tokens.
  /// 
  /// Returns an error if inline constraints are violated.
  fn tokenize(self) -> Result<Tokens, InlineError>;

  /// Takes an input and an offset to convert the input into a list of tokens,
  /// where the first token starts at the given offset.
  /// 
  /// Returns an error if inline constraints are violated.
  fn tokenize_with_offset(self, offset: Position) -> Result<Tokens, InlineError>;
}

impl Tokenizer for &str {
  fn tokenize(self) -> Result<Tokens, InlineError> {
    self.tokenize_with_offset(Position::default())
  }

  fn tokenize_with_offset(self, offset: Position) -> Result<Tokens, InlineError> {
    let mut tokenized = Tokenized::from((self, offset));
    tokenize_until(&mut tokenized, TokenKind::Eoi)?;
    // EOI is treated as newline
    update_open_map(&mut tokenized, true);
    try_closing_fixated_token(&mut tokenized, true);
    cleanup_loose_open_tokens(&mut tokenized);

    Ok(tokenized.tokens)
  }
}

/// Internal structure to keep track of the tokenization process.
#[derive(Debug)]
struct Tokenized<'a> {
  /// Input converted to a grapheme iterator.
  graphemes: Graphemes<'a>,
  /// List of tokens that were tokenized so far.
  tokens: Vec::<Token>,
  /// Map of open tokens that were not yet closed
  open_tokens: HashMap::<TokenKind, usize>,
  /// The position inside the input of the current token being tokenized.
  cur_pos: Position,
  /// Flag indicating that a grapheme must be escaped.
  escape_active: bool,
}

impl<'a> From<(&'a str, Position)> for Tokenized<'a> {
  fn from((content, offset): (&'a str, Position)) -> Self {
    Tokenized {
      graphemes: content.graphemes(true),
      tokens: Default::default(),
      open_tokens: Default::default(),
      cur_pos: offset,
      escape_active: false,
    }
  }
}

/// Function creates tokens until `token_kind` is matched, or end of input is reached.
/// 
/// Note: The token of kind `token_kind` is also included in the resulting tokens vector.
fn tokenize_until(tokenized: &mut Tokenized, token_kind: TokenKind) -> Result<(), InlineError> {
  let mut prev_tokens_len = tokenized.tokens.len();
  while let Some(grapheme) = tokenized.graphemes.next() {
    update_tokens(tokenized, grapheme)?;

    if tokenized.tokens.len() != prev_tokens_len && !tokenized.tokens.is_empty() {
      // Last token excluded, since it is not fixated yet
      let last = tokenized.tokens.pop().unwrap();
      if !last.closes_scope() {
        update_open_map(tokenized, last.is_space_or_newline());
        try_closing_fixated_token(tokenized, last.is_space_or_newline());
      }
      
      let last_kind = last.kind;
      tokenized.tokens.push(last);

      if last_kind == token_kind {
        return Ok(());
      }
    }
    prev_tokens_len = tokenized.tokens.len();
  }

  // Brackets must close
  if let Some(last) = tokenized.tokens.last() {
    if token_kind != TokenKind::Eoi && last.kind != token_kind {
      return Err(InlineError::ClosingViolation);
    }
  }

  Ok(())
}

/// Handles verbatim tokens.
fn update_accent(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }

  match tokenized.open_tokens.contains_key(&TokenKind::VerbatimOpen) {
    true => {
      let new_token = Token{ kind: TokenKind::VerbatimClose, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    },
    false => {
      let new_token = Token{ kind: TokenKind::VerbatimOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    },
  }
}

/// Updates the list of tokens by handling the next grapheme of the input.
fn update_tokens(tokenized: &mut Tokenized, grapheme: &str) -> Result<(), InlineError> {
  if tokenized.escape_active {
    update_escaped(tokenized, grapheme);
    tokenized.escape_active = false;
  } else {
    let single_token_kind = grapheme.as_single_token_kind();
    // Only single grapheme tokens need to be handled here, because only single grapheme is handled per update
    match single_token_kind {
      SingleTokenKind::Plain => update_plain(tokenized, grapheme),
      SingleTokenKind::Newline => update_newline(tokenized, grapheme),
      SingleTokenKind::Space => update_space(tokenized, grapheme),
      SingleTokenKind::Backslash => { 
        tokenized.escape_active = true;
      },
      // SingleTokenKind::ExclamationMark => todo!(),
      // SingleTokenKind::Ampersand => todo!(),
      // SingleTokenKind::Colon => todo!(),
      // SingleTokenKind::Caret => todo!(),
      // SingleTokenKind::Underscore => todo!(),
      SingleTokenKind::Asterisk => update_asterisk(tokenized, grapheme),
      // SingleTokenKind::Plus => todo!(),
      SingleTokenKind::Accent => update_accent(tokenized, grapheme),
      SingleTokenKind::LeftSquareBracket => open_text_group(tokenized, grapheme)?,
      SingleTokenKind::RightSquareBracket => try_closing_text_group(tokenized, grapheme),
    }
  }

  Ok(())
}

/// Handles text group tokenization by taking precedence over inline formattings.
/// This is achieved by recursive tokenization expecting text group close token.
/// 
/// Note: The recursive approach enforces the closing constraint.
fn open_text_group(tokenized: &mut Tokenized, grapheme: &str) -> Result<(), InlineError> {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }

  update_open_map(tokenized, false);
  try_closing_fixated_token(tokenized, false);
  
  // Makes sure to not have formattings over text group borders
  let outer_open_tokens = tokenized.open_tokens.clone();
  tokenized.open_tokens = HashMap::default();

  let new_token = Token{ kind: TokenKind::TextGroupOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
  tokenized.tokens.push(new_token);

  tokenize_until(tokenized, TokenKind::TextGroupClose)?;

  let closing_token = tokenized.tokens.pop().unwrap();
  try_closing_fixated_token(tokenized, true);
  cleanup_loose_open_tokens(tokenized);
  tokenized.tokens.push(closing_token);

  tokenized.open_tokens = outer_open_tokens;

  Ok(())
}

/// Function to close a text group if possible.
fn try_closing_text_group(tokenized: &mut Tokenized, grapheme: &str) {
  if tokenized.open_tokens.remove(&TokenKind::TextGroupOpen).is_some() {
    if let Some(last) = tokenized.tokens.last() {
      tokenized.cur_pos.column += last.length();
    }
    tokenized.tokens.push(Token{ kind: TokenKind::TextGroupClose, content: grapheme.to_string(), position: tokenized.cur_pos });
  } else if let Some(last) = tokenized.tokens.last_mut() {
    tokenized.cur_pos.column += last.length();
    let new_token = Token{ kind: TokenKind::Plain, content: grapheme.to_string(), position: tokenized.cur_pos };
    
    if last.kind == TokenKind::Plain {
      last.content.push_str(&new_token.content);
    } else {
      tokenized.tokens.push(new_token);
    }
  }
}

/// Function removes any dangling open token between open/close tokens of the last fix token, if it is a closing one.
fn try_closing_fixated_token(tokenized: &mut Tokenized, next_token_is_space_or_newline: bool) {
  if let Some(mut last) = tokenized.tokens.pop() {
    let open_index;
    let mut updated_open_tokens = HashMap::new();
    match last.kind {
        TokenKind::BoldClose => { 
          if let Some(index) = tokenized.open_tokens.remove(&TokenKind::BoldOpen) {
            open_index = index;
          } else {
            open_index = tokenized.open_tokens.remove(&TokenKind::BoldItalicOpen).expect("Closing token requires open token");
            let open_token = tokenized.tokens.get_mut(open_index).expect("Got token index from hashmap");
            open_token.kind = TokenKind::ItalicOpen;
            open_token.content = TokenKind::ItalicOpen.as_str().to_string();
            updated_open_tokens.insert(open_token.kind, open_index);
            let new_pos = Position { line: open_token.position.line, column: open_token.position.column + open_token.length() };
            // +1 because the inner token gets closed first
            tokenized.tokens.insert(open_index + 1, Token { 
              kind: TokenKind::BoldOpen, content: TokenKind::BoldOpen.as_str().to_string(), position: new_pos
            });
          }
        },
        TokenKind::ItalicClose => { 
          if let Some(index) = tokenized.open_tokens.remove(&TokenKind::ItalicOpen) {
            open_index = index;
          } else if let Some(index) = tokenized.open_tokens.remove(&TokenKind::BoldItalicOpen) {
            open_index = index;
            let open_token = tokenized.tokens.get_mut(open_index).expect("Got token index from hashmap");
            open_token.kind = TokenKind::BoldOpen;
            open_token.content = TokenKind::BoldOpen.as_str().to_string();
            updated_open_tokens.insert(open_token.kind, open_index);
            let new_pos = Position { line: open_token.position.line, column: open_token.position.column + open_token.length() };
            // +1 because the inner token gets closed first
            tokenized.tokens.insert(open_index + 1, Token { 
              kind: TokenKind::ItalicOpen, content: TokenKind::ItalicOpen.as_str().to_string(), position: new_pos
            });
          } else {
            // ItalicClose kept open for possible BoldClose, but stayed at ItalicClose
            if next_token_is_space_or_newline {
              last.kind = TokenKind::Plain;
              if let Some(prev) = tokenized.tokens.last_mut() {
                if prev.kind == TokenKind::Plain {
                  prev.content.push_str(&last.content);
                  return;
                }
              }
            } else {
              last.kind = TokenKind::ItalicOpen;
              tokenized.open_tokens.insert(last.kind, tokenized.tokens.len());
            }
            tokenized.tokens.push(last);
            return;
          }
        },
        TokenKind::BoldItalicClose => { 
          if let Some(index) = tokenized.open_tokens.remove(&TokenKind::BoldItalicOpen) {
            open_index = index;
          } else {
            let bold_index = tokenized.open_tokens.remove(&TokenKind::BoldOpen).expect("Bold open must exist for bold-italic closing");
            let italic_index = tokenized.open_tokens.remove(&TokenKind::ItalicOpen).expect("Italic open must exist for bold-italic closing");
            open_index = min(bold_index, italic_index);
          }
        },
        TokenKind::VerbatimClose => { open_index = tokenized.open_tokens.remove(&TokenKind::VerbatimOpen).unwrap(); },
        // TokenKind::EmojiClose => { open_index = tokenized.open_tokens.remove(&TokenKind::EmojiOpen).unwrap(); },
        // TokenKind::CommentClose => { open_index = tokenized.open_tokens.remove(&TokenKind::CommentOpen).unwrap(); },
        _ => { 
          tokenized.tokens.push(last);
          return; 
        },
    }

    tokenized.tokens.push(last);

    for (kind, index) in &tokenized.open_tokens.clone() {
      if *index < open_index {
        updated_open_tokens.insert(*kind, *index);
      } else if tokenized.tokens.len() > *index {
        try_plain_token_merge(tokenized, *index);
      }
    }
    tokenized.open_tokens = updated_open_tokens;
  }
}

/// Enteres the last fixed token into the open token hashmap, if it is an open token.
/// 
/// Note: Enforces open token contraints, changing a token to plain if a constraint is violated.
fn update_open_map(tokenized: &mut Tokenized, next_token_is_space_or_newline: bool) {
  if let Some(mut prev) = tokenized.tokens.pop() {
    // Makes sure that no two open tokens of the same kind are before one closing one
    if let Vacant(e) = tokenized.open_tokens.entry(prev.kind) {
      match prev.kind {
        TokenKind::BoldOpen
        | TokenKind::ItalicOpen
        | TokenKind::BoldItalicOpen
        // | TokenKind::CommentOpen
        | TokenKind::EmojiOpen => {
          if next_token_is_space_or_newline {
            prev.kind = TokenKind::Plain;
          } else {
            e.insert(tokenized.tokens.len());
          }
        },
        TokenKind::VerbatimOpen
        | TokenKind::TextGroupOpen => { e.insert(tokenized.tokens.len()); },
        _ => {  },
      }
    } else {
      prev.kind = TokenKind::Plain;
    }

    // Try plain merge
    if let Some(prev_prev) = tokenized.tokens.last_mut() {
      if prev_prev.kind == TokenKind::Plain && prev.kind == TokenKind::Plain {
        prev_prev.content.push_str(&prev.content);
      } else {
        tokenized.tokens.push(prev);
      }
    } else {
      tokenized.tokens.push(prev);
    }
  }
}

/// Handles plain text.
fn update_plain(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last_mut() {
    if last.kind == TokenKind::Plain {
      last.content.push_str(grapheme);
    } else {
      tokenized.cur_pos.column += last.length();
      let new_token = Token{ kind: TokenKind::Plain, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    }
  } else {
    let new_token = Token{ kind: TokenKind::Plain, content: grapheme.to_string(), position: tokenized.cur_pos };
    tokenized.tokens.push(new_token);
  }
}

/// Handles escaped graphemes.
fn update_escaped(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }
  tokenized.tokens.push(Token{ kind: TokenKind::EscapedGrapheme, content: grapheme.to_string(), position: tokenized.cur_pos });
  tokenized.cur_pos.column += 1; // add backslash length offset for next token start
}

/// Handles graphemes with Unicode whitespace property that are not a newline.
fn update_space(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last_mut() {
    if last.kind == TokenKind::Space {
      last.content.push_str(grapheme);
    } else {
      tokenized.cur_pos.column += last.length();
      let new_token = Token{ kind: TokenKind::Space, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    }
  } else {
    let new_token = Token{ kind: TokenKind::Space, content: grapheme.to_string(), position: tokenized.cur_pos };
    tokenized.tokens.push(new_token);
  }
}

/// Handles newlines.
fn update_newline(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }

  let new_token = Token{ kind: TokenKind::NewLine, content: grapheme.to_string(), position: tokenized.cur_pos };
  tokenized.tokens.push(new_token);
  tokenized.cur_pos.line += 1;
  tokenized.cur_pos.column = 0;
}

/// Handles bold, italic and any combination of them.
fn update_asterisk(tokenized: &mut Tokenized, grapheme: &str) {
  match tokenized.tokens.pop() {
    Some(mut last) => {
      if last.kind == TokenKind::ItalicOpen {
        last.content.push_str(grapheme);

        if tokenized.open_tokens.get(&TokenKind::BoldOpen).is_some() {
          let preceding_token = tokenized.tokens.last().expect("Tokens must not be empty, because open token exists");
          if preceding_token.is_space_or_newline() {
            // Close after space is not allowed
            last.kind = TokenKind::Plain;
          } else {
            last.kind = TokenKind::BoldClose;
          }
        } else {
          last.kind = TokenKind::BoldOpen;
        }
        tokenized.tokens.push(last);    
      } else if last.kind == TokenKind::BoldOpen {
        if tokenized.open_tokens.get(&TokenKind::ItalicOpen).is_some() {
          // Handles cases like `*italic***bold**`
          let preceding_token = tokenized.tokens.last().expect("Tokens must not be empty, because open token exists");
          if preceding_token.is_space_or_newline() {
            // If Space is before `***`, it is split into [plain|italicClose|italicOpen] -> `*before ***after*` = `[io]before *[ic][io]after[ic]
            last.kind = TokenKind::Plain;
            last.content = TokenKind::ItalicOpen.as_str().to_string();
            tokenized.cur_pos.column += last.length();
            tokenized.tokens.push(last);

            let italic_close_token = Token { kind: TokenKind::ItalicClose, content: TokenKind::ItalicClose.as_str().to_string(), position: tokenized.cur_pos };
            tokenized.cur_pos.column += italic_close_token.length();
            tokenized.tokens.push(italic_close_token);
  
            let italic_open_token = Token { kind: TokenKind::ItalicOpen, content: TokenKind::ItalicClose.as_str().to_string(), position: tokenized.cur_pos };
            tokenized.tokens.push(italic_open_token);
          } else {
            last.kind = TokenKind::ItalicClose;
            last.content = TokenKind::ItalicClose.as_str().to_string();
            tokenized.cur_pos.column += last.length();
            tokenized.tokens.push(last);
  
            let bold_open_token = Token { kind: TokenKind::BoldOpen, content: TokenKind::BoldOpen.as_str().to_string(), position: tokenized.cur_pos };
            tokenized.tokens.push(bold_open_token);
          }  
        } else {
          last.kind = TokenKind::BoldItalicOpen;
          last.content.push_str(grapheme);
          tokenized.tokens.push(last);
        }
      } else if last.kind == TokenKind::BoldItalicOpen {
        // Handles `****` by converting the leftmost `*` to plain.
        // If no italic, bold or bolditalic open token is present before, bolditalicopen is kept as is.
        // Otherwise, italic, bold or bolditalic closing tokens are taken from the remaining three `*`.
        last.kind = TokenKind::Plain;
        last.content = TokenKind::ItalicOpen.as_str().to_string();
        tokenized.cur_pos.column += last.length();

        if (tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen) && tokenized.open_tokens.contains_key(&TokenKind::BoldOpen))
          || tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {

          tokenized.tokens.push(last);

          let combined_close_token = Token { kind: TokenKind::BoldItalicClose, content: TokenKind::BoldItalicClose.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(combined_close_token);
        } else if tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen) {
          tokenized.tokens.push(last);

          let italic_close_token = Token { kind: TokenKind::ItalicClose, content: TokenKind::ItalicClose.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(italic_close_token);

          let bold_open_token = Token { kind: TokenKind::BoldOpen, content: TokenKind::BoldOpen.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(bold_open_token);
        } else if tokenized.open_tokens.contains_key(&TokenKind::BoldOpen) {
          tokenized.tokens.push(last);

          let bold_close_token = Token { kind: TokenKind::BoldClose, content: TokenKind::BoldClose.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.cur_pos.column += bold_close_token.length();
          tokenized.tokens.push(bold_close_token);

          let italic_open_token = Token { kind: TokenKind::ItalicOpen, content: TokenKind::ItalicOpen.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(italic_open_token);
        } else {
          match tokenized.tokens.last_mut() {
            Some(prev) => {
              if prev.kind == TokenKind::Plain {
                prev.content.push_str(&last.content);
              } else {
                tokenized.tokens.push(last);
              }
            },
            None => {
              tokenized.tokens.push(last);
            },
          }

          let combined_open_token = Token { kind: TokenKind::BoldItalicOpen, content: TokenKind::BoldItalicOpen.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(combined_open_token);
        }
      } else if last.kind == TokenKind::ItalicClose {
        if tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen)
          || tokenized.open_tokens.contains_key(&TokenKind::BoldOpen) {
          last.kind = TokenKind::BoldClose;
          last.content.push_str(grapheme);
          tokenized.tokens.push(last);
        } else {
          last.kind = TokenKind::BoldOpen;
          last.content.push_str(grapheme);
          tokenized.tokens.push(last);
        }
      } else if last.kind == TokenKind::BoldClose {
        if tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {
          last.content.push_str(grapheme);
          last.kind = TokenKind::BoldItalicClose;
          tokenized.tokens.push(last);
        } else {
          match tokenized.open_tokens.get(&TokenKind::ItalicOpen) {
            Some(italic_index) => {
              let bold_index = tokenized.open_tokens.get(&TokenKind::BoldOpen).unwrap();
              if italic_index < bold_index {
                last.kind = TokenKind::BoldClose;
                last.content = TokenKind::BoldClose.as_str().to_string();
                tokenized.cur_pos.column += last.length();
                tokenized.tokens.push(last);
                let new_token = Token{
                  kind: TokenKind::ItalicClose, 
                  content: TokenKind::ItalicClose.as_str().to_string(),
                  position: tokenized.cur_pos
                };
                tokenized.tokens.push(new_token);
              } else {
                last.kind = TokenKind::ItalicClose;
                last.content = TokenKind::ItalicClose.as_str().to_string();
                tokenized.cur_pos.column += last.length();
                tokenized.tokens.push(last);
                let new_token = Token{
                  kind: TokenKind::BoldClose, 
                  content: TokenKind::BoldClose.as_str().to_string(),
                  position: tokenized.cur_pos
                };
                tokenized.tokens.push(new_token);
              }
            },
            None => {
              // Handles `**bold***italic*` -> [bo]bold[bc][io]italic[ic]
              tokenized.cur_pos.column += last.length();
              tokenized.tokens.push(last);
              let new_token = Token{ kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
              tokenized.tokens.push(new_token);
            }
          }
        }
      } else if last.kind == TokenKind::BoldItalicClose {
        // Handles `***bold & italic****italic*` -> [bio]bold & italic[bic][io]italic[ic]
        tokenized.cur_pos.column += last.length();
        tokenized.tokens.push(last);
        let new_token = Token{ kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
        tokenized.tokens.push(new_token);
      } else {
        let new_token;
        tokenized.cur_pos.column += last.length();
        if tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen)
          || tokenized.open_tokens.contains_key(&TokenKind::BoldOpen)
          || tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {

          if last.is_space_or_newline() {
            // Closing not allowed after space
            new_token = Token{ kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
          } else {
            new_token = Token{ kind: TokenKind::ItalicClose, content: grapheme.to_string(), position: tokenized.cur_pos };
          }
        } else {
          new_token = Token{ kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
        }

        tokenized.tokens.push(last);
        tokenized.tokens.push(new_token);
      }
    },
    None => {
      let new_token = Token{ kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    },
  }
}

/// Cleans up open tokens.
/// 
/// Remaining open tokens that have no matching close token get converted to plain.
/// Neighboring plain tokens get merged with the open token. 
fn cleanup_loose_open_tokens(tokenized: &mut Tokenized) {
  let open_tokens = tokenized.open_tokens.clone();
  let mut open_indizes: Vec<_> = open_tokens.values().collect();
  open_indizes.sort();
  open_indizes.reverse();

  for index in open_indizes {
    try_plain_token_merge(tokenized, *index);
  }
}

/// Function that tries to convert a token to `Plain`
/// and merge it with previous and/or next token, if they are also `Plain`.
fn try_plain_token_merge(tokenized: &mut Tokenized, index: usize) {
  if index >= tokenized.tokens.len() {
    return;
  }

  let mut token = tokenized.tokens.remove(index);
  token.kind = TokenKind::Plain;
  if index < tokenized.tokens.len() {
    let next_token = tokenized.tokens.remove(index);
    if next_token.kind == TokenKind::Plain {
      token.content.push_str(&next_token.content);
    } else {
      tokenized.tokens.insert(index, next_token);
    }
  }

  if index > 0 {
    if let Some(prev_token) = tokenized.tokens.get_mut(index - 1) {
      if prev_token.kind == TokenKind::Plain {
        prev_token.content.push_str(&token.content);
      } else {
        tokenized.tokens.insert(index, token);
      }
    } else {
      tokenized.tokens.insert(index, token);
    }
  } else {
    tokenized.tokens.insert(index, token);
  }
}
