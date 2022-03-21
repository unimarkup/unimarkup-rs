use std::{collections::{HashMap, hash_map::Entry::Vacant}, cmp::min};

use unicode_segmentation::{UnicodeSegmentation, Graphemes};

use crate::{Position, error::InlineError};

use super::tokens::{Token, TokenKind, AsSingleTokenKind, SingleTokenKind, Tokens};

pub(crate) trait Tokenizer {
  fn tokenize(self) -> Result<Tokens, InlineError>;
}

impl Tokenizer for &str {
  fn tokenize(self) -> Result<Tokens, InlineError> {
    let mut tokenized = Tokenized::from(self);
    tokenize_until(&mut tokenized, TokenKind::Eoi)?;
    // EOI is treated as newline
    update_open_map(&mut tokenized, true);
    try_closing_fixated_token(&mut tokenized);
    cleanup_loose_open_tokens(&mut tokenized);

    Ok(tokenized.tokens)
  }
}

#[derive(Debug)]
struct Tokenized<'a> {
  graphemes: Graphemes<'a>,
  tokens: Vec::<Token>,
  open_tokens: HashMap::<TokenKind, usize>,
  cur_pos: Position,
  escape_active: bool,
  open_verbatim: bool,
}

impl<'a> From<&'a str> for Tokenized<'a> {
  fn from(content: &'a str) -> Self {
    Tokenized {
      graphemes: content.graphemes(true),
      tokens: Default::default(),
      open_tokens: Default::default(),
      cur_pos: Default::default(),
      escape_active: false,
      open_verbatim: false,
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
        try_closing_fixated_token(tokenized);
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
      return Err(InlineError{});
    }
  }

  Ok(())
}

fn update_accent(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }

  match tokenized.open_tokens.contains_key(&TokenKind::VerbatimOpen) {
    true => {
      let new_token = Token{ kind: TokenKind::VerbatimClose, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
      tokenized.open_verbatim = false;
    },
    false => {
      let new_token = Token{ kind: TokenKind::VerbatimOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
      tokenized.open_verbatim = true;
    },
  }
}


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
      SingleTokenKind::ExclamationMark => todo!(),
      SingleTokenKind::Ampersand => todo!(),
      SingleTokenKind::Colon => todo!(),
      SingleTokenKind::Caret => todo!(),
      SingleTokenKind::Underscore => todo!(),
      SingleTokenKind::Asterisk => update_asterisk(tokenized, grapheme),
      SingleTokenKind::Plus => todo!(),
      SingleTokenKind::Accent => update_accent(tokenized, grapheme),
      SingleTokenKind::LeftSquareBracket => open_text_group(tokenized, grapheme)?,
      SingleTokenKind::RightSquareBracket => try_closing_text_group(tokenized, grapheme),
    }
  }

  Ok(())
}

fn open_text_group(tokenized: &mut Tokenized, grapheme: &str) -> Result<(), InlineError> {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }

  update_open_map(tokenized, false);
  try_closing_fixated_token(tokenized);
  
  // Makes sure to not have formattings over text group borders
  let outer_open_tokens = tokenized.open_tokens.clone();
  tokenized.open_tokens = HashMap::default();

  let new_token = Token{ kind: TokenKind::TextGroupOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
  tokenized.tokens.push(new_token);

  tokenize_until(tokenized, TokenKind::TextGroupClose)?;

  let closing_token = tokenized.tokens.pop().unwrap();
  try_closing_fixated_token(tokenized);
  cleanup_loose_open_tokens(tokenized);
  tokenized.tokens.push(closing_token);

  tokenized.open_tokens = outer_open_tokens;

  Ok(())
}

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


/// Function removes any dangling open token between open/close tokens of the last fix token, if it is a closing one
fn try_closing_fixated_token(tokenized: &mut Tokenized) {
  if let Some(last) = tokenized.tokens.last() {
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
          } else {
            open_index = tokenized.open_tokens.remove(&TokenKind::BoldItalicOpen).expect("Closing token requires open token");
            let open_token = tokenized.tokens.get_mut(open_index).expect("Got token index from hashmap");
            open_token.kind = TokenKind::BoldOpen;
            open_token.content = TokenKind::BoldOpen.as_str().to_string();
            updated_open_tokens.insert(open_token.kind, open_index);
            let new_pos = Position { line: open_token.position.line, column: open_token.position.column + open_token.length() };
            // +1 because the inner token gets closed first
            tokenized.tokens.insert(open_index + 1, Token { 
              kind: TokenKind::ItalicOpen, content: TokenKind::ItalicOpen.as_str().to_string(), position: new_pos
            });
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
        TokenKind::EmojiClose => { open_index = tokenized.open_tokens.remove(&TokenKind::EmojiOpen).unwrap(); },
        TokenKind::CommentClose => { open_index = tokenized.open_tokens.remove(&TokenKind::CommentOpen).unwrap(); },
        _ => { return; },
    }

    for (kind, index) in &tokenized.open_tokens {
      if *index < open_index {
        updated_open_tokens.insert(*kind, *index);
      }
    }
    tokenized.open_tokens = updated_open_tokens;
  }
}

/// Enteres the last fixed token into the open token hashmap, if it is an open token.
/// 
/// Note: Enforces open token contraints, changing a token to plain if a constraint is violated
fn update_open_map(tokenized: &mut Tokenized, next_token_is_space_or_newline: bool) {
  if let Some(mut prev) = tokenized.tokens.pop() {
    // Makes sure that no two open tokens of the same kind are before one closing one
    if let Vacant(e) = tokenized.open_tokens.entry(prev.kind) {
      match prev.kind {
        TokenKind::BoldOpen
        | TokenKind::ItalicOpen
        | TokenKind::BoldItalicOpen
        | TokenKind::CommentOpen
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

fn update_escaped(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }
  tokenized.tokens.push(Token{ kind: TokenKind::EscapedGrapheme, content: grapheme.to_string(), position: tokenized.cur_pos });
  tokenized.cur_pos.column += 1; // add backslash length offset for next token start
}

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

fn update_newline(tokenized: &mut Tokenized, grapheme: &str) {
  if let Some(last) = tokenized.tokens.last() {
    tokenized.cur_pos.column += last.length();
  }

  let new_token = Token{ kind: TokenKind::NewLine, content: grapheme.to_string(), position: tokenized.cur_pos };
  tokenized.tokens.push(new_token);
  tokenized.cur_pos.line += 1;
  tokenized.cur_pos.column = 0;
}

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
        if tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {
          last.kind = TokenKind::BoldClose;
          last.content.push_str(grapheme);
          tokenized.tokens.push(last);
        } else if let Some(bold_index) = tokenized.open_tokens.get(&TokenKind::BoldOpen) {
          match tokenized.open_tokens.get(&TokenKind::ItalicOpen) {
            Some(italic_index) => {
              if italic_index < bold_index {
                last.kind = TokenKind::BoldClose;
                last.content.push_str(grapheme);
                tokenized.tokens.push(last);
              } else {
                last.kind = TokenKind::ItalicClose;
                tokenized.cur_pos.column += last.length();
                tokenized.tokens.push(last);
                tokenized.tokens.push(Token { 
                  kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos 
                })
              }
            },
            None => { 
              last.kind = TokenKind::BoldClose;
              last.content.push_str(grapheme); 
              tokenized.tokens.push(last);
            },
          }
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
          // Handles `**bold***italic*` -> [bo]bold[bc][io]italic[ic]
          tokenized.cur_pos.column += last.length();
          tokenized.tokens.push(last);
          let new_token = Token{ kind: TokenKind::ItalicOpen, content: grapheme.to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(new_token);
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

/// Remaining open tokens that have no matching close token get converted to plain.
/// Neighboring plain tokens get merged with the open token. 
fn cleanup_loose_open_tokens(tokenized: &mut Tokenized) {
  let mut open_indizes: Vec<_> = tokenized.open_tokens.values().collect();
  open_indizes.sort();

  for index in open_indizes {
    let mut token = tokenized.tokens.remove(*index);
    token.kind = TokenKind::Plain;
    if (*index) < tokenized.tokens.len() {
      let next_token = tokenized.tokens.remove(*index);
      if next_token.kind == TokenKind::Plain {
        token.content.push_str(&next_token.content);
      } else {
        tokenized.tokens.insert(*index, next_token);
      }
    }

    if *index > 0 {
      if let Some(prev_token) = tokenized.tokens.get_mut(*index - 1) {
        if prev_token.kind == TokenKind::Plain {
          prev_token.content.push_str(&token.content);
        } else {
          tokenized.tokens.insert(*index, token);
        }
      } else {
        tokenized.tokens.insert(*index, token);
      }
    } else {
      tokenized.tokens.insert(*index, token);
    }
  }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
  use super::*;

  pub const EXPECTED_MSG: &str = "actual(left) != expected(right)";

  #[test]
  pub fn test_formatting__plain_before_italic() {
    let input = "plain text *italic*";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 12 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 18 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__plain_after_bold() {
    let input = "**bold** plain text";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 14 } },
      Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 15 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__right_side_nested() {
    let input = "**bold and *italic***";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Plain, content: "and".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 12 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 18 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 19 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__left_side_nested() {
    let input = "***italic* and bold**";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::Plain, content: "and".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 14 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 15 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 19 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__left_side_nested_with_plain_ending() {
    let input = "***italic* and bold** plain";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::Plain, content: "and".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 14 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 15 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 19 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 21 } },
      Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 22 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__escape_open_italic() {
    let input = "\\*not italic*";
    let expected = [
      Token{ kind: TokenKind::EscapedGrapheme, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "not".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::Plain, content: "italic*".to_string(), position: Position { line: 0, column: 6 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__bold_directly_after_italic() {
    let input = "*italic***bold**";
    let expected = [
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 14 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
  
  #[test]
  pub fn test_formatting__split_bold_italic_combined_close_due_to_space() {
    let input = "*before ***after*";
    let expected = [
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "before".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "*".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::Plain, content: "after".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 16 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__asterisks_as_plain() {
    let input = "before****after";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "before****after".to_string(), position: Position { line: 0, column: 0 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__asterisks_as_plain_surrounded_by_space() {
    let input = "before **** after";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "before".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Plain, content: "****".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "after".to_string(), position: Position { line: 0, column: 12 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
  
  #[test]
  pub fn test_formatting__italic_directly_after_bold() {
    let input = "**bold***italic*";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 15 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
   
  #[test]
  pub fn test_formatting__italic_directly_after_combined_bold_italic() {
    let input = "***bold & italic****italic*";
    let expected = [
      Token{ kind: TokenKind::BoldItalicOpen, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "&".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 10 } },
      Token{ kind: TokenKind::BoldItalicClose, content: "***".to_string(), position: Position { line: 0, column: 16 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 19 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 20 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 26 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__italic_directly_after_plain_asterisks() {
    let input = "****italic*";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::ItalicOpen, content: "*".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Plain, content: "italic".to_string(), position: Position { line: 0, column: 4 } },
      Token{ kind: TokenKind::ItalicClose, content: "*".to_string(), position: Position { line: 0, column: 10 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__bold_directly_after_plain_asterisks() {
    let input = "*****bold**";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 9 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__combined_directly_after_plain_asterisks() {
    let input = "******bold-italic***";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "***".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::BoldItalicOpen, content: "***".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::Plain, content: "bold-italic".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::BoldItalicClose, content: "***".to_string(), position: Position { line: 0, column: 17 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__plain_asterisks() {
    let input = "*********";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "*********".to_string(), position: Position { line: 0, column: 0 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__invalid_italic_open() {
    let input = "* no italic*";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "*".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Plain, content: "no".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 4 } },
      Token{ kind: TokenKind::Plain, content: "italic*".to_string(), position: Position { line: 0, column: 5 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__invalid_bold_open() {
    let input = "plain** still plain**";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "plain**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "still".to_string(), position: Position { line: 0, column: 8 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 13 } },
      Token{ kind: TokenKind::Plain, content: "plain**".to_string(), position: Position { line: 0, column: 14 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__verbatim() {
    let input = "`verbatim`";
    let expected = [
      Token{ kind: TokenKind::VerbatimOpen, content: "`".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "verbatim".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::VerbatimClose, content: "`".to_string(), position: Position { line: 0, column: 9 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__verbatim_escaped_close() {
    let input = "`verbatim\\`still verbatim`";
    let expected = [
      Token{ kind: TokenKind::VerbatimOpen, content: "`".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "verbatim".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::EscapedGrapheme, content: "`".to_string(), position: Position { line: 0, column: 9 } },
      Token{ kind: TokenKind::Plain, content: "still".to_string(), position: Position { line: 0, column: 11 } },     
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 16 } },
      Token{ kind: TokenKind::Plain, content: "verbatim".to_string(), position: Position { line: 0, column: 17 } },
      Token{ kind: TokenKind::VerbatimClose, content: "`".to_string(), position: Position { line: 0, column: 25 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__simple_text_group() {
    let input = "[valid text group]";
    let expected = [
      Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "valid".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 7 } },     
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "group".to_string(), position: Position { line: 0, column: 12 } },
      Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 17 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__plain_before_text_group() {
    let input = "plain[valid text group]";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::Plain, content: "valid".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 12 } },     
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 16 } },
      Token{ kind: TokenKind::Plain, content: "group".to_string(), position: Position { line: 0, column: 17 } },
      Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 22 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__plain_after_text_group() {
    let input = "[valid text group]plain";
    let expected = [
      Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "valid".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 6 } },
      Token{ kind: TokenKind::Plain, content: "text".to_string(), position: Position { line: 0, column: 7 } },     
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 11 } },
      Token{ kind: TokenKind::Plain, content: "group".to_string(), position: Position { line: 0, column: 12 } },
      Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 17 } },
      Token{ kind: TokenKind::Plain, content: "plain".to_string(), position: Position { line: 0, column: 18 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__formatting_inside_text_group() {
    let input = "[**bold**]";
    let expected = [
      Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 3 } },     
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 9 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__invalid_formatting_over_text_group_borders() {
    let input = "[**bold]**";
    let expected = [
      Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::Plain, content: "**bold".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::Plain, content: "**".to_string(), position: Position { line: 0, column: 8 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__formatting_outside_text_group() {
    let input = "**[bold]**";
    let expected = [
      Token{ kind: TokenKind::BoldOpen, content: "**".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::TextGroupOpen, content: "[".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Plain, content: "bold".to_string(), position: Position { line: 0, column: 3 } },
      Token{ kind: TokenKind::TextGroupClose, content: "]".to_string(), position: Position { line: 0, column: 7 } },
      Token{ kind: TokenKind::BoldClose, content: "**".to_string(), position: Position { line: 0, column: 8 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_tokenize__two_plain_lines() {
    let input = "line1\nline2";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "line1".to_string(), position: Position { line: 0, column: 0 } },
      Token{ kind: TokenKind::NewLine, content: "\n".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::Plain, content: "line2".to_string(), position: Position { line: 1, column: 0 } },
    ];

    let actual = input.tokenize().unwrap();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

}

