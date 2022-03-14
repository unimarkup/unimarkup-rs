use std::{collections::{HashMap, hash_map::Entry::Vacant}, cmp::min};

use crate::Position;

use super::tokens::{Token, TokenKind, AsSingleTokenKind, SingleTokenKind};


#[derive(Debug, Default)]
struct Tokenized {
  tokens: Vec::<Token>,
  open_tokens: HashMap::<TokenKind, usize>,
  cur_pos: Position,
  escape_active: bool,
}

pub(crate) trait Tokenizer {
  fn tokenize(self) -> Vec<Token>;
}

impl Tokenizer for &str {
  fn tokenize(self) -> Vec<Token> {
    let mut tokenized = Tokenized::default();
    let mut last_token_index = 0;
    
    for c in self.chars() {
      update_tokens(&mut tokenized, c);

      let updated_last_token_index = if tokenized.tokens.is_empty() { 0 } else { tokenized.tokens.len() - 1 };
      if last_token_index != updated_last_token_index && updated_last_token_index > 0 {
        // Note: last token excluded, since it is not fixated yet
        let last = tokenized.tokens.pop().unwrap();
        update_open_map(&mut tokenized, last.is_space_or_newline(), last_token_index);
        handle_last_closing_token(&mut tokenized);
        tokenized.tokens.push(last);

        last_token_index = updated_last_token_index;
      }
    }
    // Note: EOI is treated as newline
    update_open_map(&mut tokenized, true, last_token_index);
    handle_last_closing_token(&mut tokenized);
    cleanup_loose_open_tokens(&mut tokenized);

    tokenized.tokens
  }
}

fn update_tokens(tokenized: &mut Tokenized, c: char) {
  if tokenized.escape_active {
    update_escaped(tokenized, c);
    tokenized.escape_active = false;
  } else {
    let single_token_kind = c.as_single_token_kind();
    // only single char tokens need to be handled here, because `c` is only one char
    match single_token_kind {
      SingleTokenKind::Plain => update_plain(tokenized, c),
      SingleTokenKind::LineFeed | SingleTokenKind::CarriageReturn => todo!(),
      SingleTokenKind::Tab => todo!(),
      SingleTokenKind::Space => update_space(tokenized, c),
      SingleTokenKind::Backslash => { 
        tokenized.escape_active = true;
        tokenized.cur_pos.column += 1;
      },
      SingleTokenKind::ExclamationMark => todo!(),
      SingleTokenKind::Ampersand => todo!(),
      SingleTokenKind::Colon => todo!(),
      SingleTokenKind::Caret => todo!(),
      SingleTokenKind::Underscore => todo!(),
      SingleTokenKind::Asterisk => update_asterisk(tokenized, c),
      SingleTokenKind::Plus => todo!(),
    }
  }
}

/// Function removes any dangling open token between open/close tokens of the last fix token, if it is a closing one
fn handle_last_closing_token(tokenized: &mut Tokenized) {
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
            // Note: +1 because the inner token gets closed first
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
            // Note: +1 because the inner token gets closed first
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
fn update_open_map(tokenized: &mut Tokenized, next_token_is_space_or_newline: bool, last_token_index: usize) {
  if let Some(mut prev) = tokenized.tokens.pop() {
    // Note: Makes sure that no two open tokens of the same kind are before one closing one
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
            e.insert(last_token_index);
          }
        },
        TokenKind::VerbatimOpen => { e.insert(last_token_index); },
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

fn update_plain(tokenized: &mut Tokenized, c: char) {
  if let Some(last) = tokenized.tokens.last_mut() {
    if last.kind == TokenKind::Plain {
      last.content.push(c);
    } else {
      tokenized.cur_pos.column += last.length();
      let new_token = Token{ kind: TokenKind::Plain, content: c.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    }
  } else {
    let new_token = Token{ kind: TokenKind::Plain, content: c.to_string(), position: tokenized.cur_pos };
    tokenized.tokens.push(new_token);
  }
}

fn update_escaped(tokenized: &mut Tokenized, c: char) {
  tokenized.tokens.push(Token{ kind: TokenKind::EscapedChar, content: c.to_string(), position: tokenized.cur_pos });
}

fn update_space(tokenized: &mut Tokenized, c: char) {
  if let Some(last) = tokenized.tokens.last_mut() {
    if last.kind == TokenKind::Space {
      last.content.push(c);
    } else {
      tokenized.cur_pos.column += last.length();
      let new_token = Token{ kind: TokenKind::Space, content: c.to_string(), position: tokenized.cur_pos };
      tokenized.tokens.push(new_token);
    }
  } else {
    let new_token = Token{ kind: TokenKind::Space, content: c.to_string(), position: tokenized.cur_pos };
    tokenized.tokens.push(new_token);
  }
}

fn update_asterisk(tokenized: &mut Tokenized, c: char) {
  match tokenized.tokens.pop() {
    Some(mut last) => {
      if last.kind == TokenKind::ItalicOpen {
        last.content.push(c);

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
          // Note: handles cases like `*italic***bold**`
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
          last.content.push(c);
          tokenized.tokens.push(last);
        }
      } else if last.kind == TokenKind::BoldItalicOpen {
        // Note: handles `****` as plain, if no italic, bold or bolditalic open token is present
        last.kind = TokenKind::Plain;
        if (tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen) && tokenized.open_tokens.contains_key(&TokenKind::BoldOpen))
          || tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {
          
          last.content = TokenKind::ItalicOpen.as_str().to_string();
          tokenized.cur_pos.column += last.length();
          tokenized.tokens.push(last);

          let combined_close_token = Token { kind: TokenKind::BoldItalicClose, content: TokenKind::BoldItalicClose.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(combined_close_token);
        } else if tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen) {
          last.content = TokenKind::ItalicOpen.as_str().to_string();
          tokenized.cur_pos.column += last.length();
          tokenized.tokens.push(last);

          let italic_close_token = Token { kind: TokenKind::ItalicClose, content: TokenKind::ItalicClose.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.cur_pos.column += italic_close_token.length();
          tokenized.tokens.push(italic_close_token);

          let bold_open_token = Token { kind: TokenKind::BoldOpen, content: TokenKind::BoldOpen.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(bold_open_token);
        } else if tokenized.open_tokens.contains_key(&TokenKind::BoldOpen) {
          last.content = TokenKind::ItalicOpen.as_str().to_string();
          tokenized.cur_pos.column += last.length();
          tokenized.tokens.push(last);

          let bold_close_token = Token { kind: TokenKind::BoldClose, content: TokenKind::BoldClose.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.cur_pos.column += bold_close_token.length();
          tokenized.tokens.push(bold_close_token);

          let italic_open_token = Token { kind: TokenKind::ItalicOpen, content: TokenKind::ItalicOpen.as_str().to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(italic_open_token);
        } else {
          last.kind = TokenKind::Plain;
          last.content.push(c);
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
        }
      } else if last.kind == TokenKind::ItalicClose {
        if tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {
          last.kind = TokenKind::BoldClose;
          last.content.push(c);
          tokenized.tokens.push(last);
        } else if let Some(bold_index) = tokenized.open_tokens.get(&TokenKind::BoldOpen) {
          match tokenized.open_tokens.get(&TokenKind::ItalicOpen) {
            Some(italic_index) => {
              if italic_index < bold_index {
                last.kind = TokenKind::BoldClose;
                last.content.push(c);
                tokenized.tokens.push(last);
              } else {
                last.kind = TokenKind::ItalicClose;
                tokenized.cur_pos.column += last.length();
                tokenized.tokens.push(last);
                tokenized.tokens.push(Token { 
                  kind: TokenKind::ItalicOpen, content: c.to_string(), position: tokenized.cur_pos 
                })
              }
            },
            None => { 
              last.kind = TokenKind::BoldClose;
              last.content.push(c); 
              tokenized.tokens.push(last);
            },
          }
        } else {
          last.kind = TokenKind::BoldOpen;
          last.content.push(c);
          tokenized.tokens.push(last);
        }
      } else if last.kind == TokenKind::BoldClose {
        if tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {
          last.content.push(c);
          last.kind = TokenKind::BoldItalicClose;
          tokenized.tokens.push(last);
        } else {
          // Note: handles `**bold***italic*` -> [bo]bold[bc][io]italic[ic]
          tokenized.cur_pos.column += last.length();
          tokenized.tokens.push(last);
          let new_token = Token{ kind: TokenKind::ItalicOpen, content: c.to_string(), position: tokenized.cur_pos };
          tokenized.tokens.push(new_token);
        }
      } else if last.kind == TokenKind::BoldItalicClose {
        // Note: handles `***bold & italic****italic*` -> [bio]bold & italic[bic][io]italic[ic]
        tokenized.cur_pos.column += last.length();
        tokenized.tokens.push(last);
        let new_token = Token{ kind: TokenKind::ItalicOpen, content: c.to_string(), position: tokenized.cur_pos };
        tokenized.tokens.push(new_token);
      } else {
        let new_token;
        tokenized.cur_pos.column += last.length();
        if tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen)
          || tokenized.open_tokens.contains_key(&TokenKind::BoldOpen)
          || tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {

          if last.is_space_or_newline() {
            // Note: closing not allowed after space
            new_token = Token{ kind: TokenKind::ItalicOpen, content: c.to_string(), position: tokenized.cur_pos };
          } else {
            new_token = Token{ kind: TokenKind::ItalicClose, content: c.to_string(), position: tokenized.cur_pos };
          }
        } else {
          new_token = Token{ kind: TokenKind::ItalicOpen, content: c.to_string(), position: tokenized.cur_pos };
        }

        tokenized.tokens.push(last);
        tokenized.tokens.push(new_token);
      }
    },
    None => {
      let new_token = Token{ kind: TokenKind::ItalicOpen, content: c.to_string(), position: tokenized.cur_pos };
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
    if (*index + 1) < tokenized.open_tokens.len() {
      let next_token = tokenized.tokens.remove(*index + 1);
      if next_token.kind == TokenKind::Plain {
        token.content.push_str(&next_token.content);
      } else {
        tokenized.tokens.insert(*index + 1, next_token);
      }
    }

    if *index > 0 {
      if let Some(prev_token) = tokenized.tokens.get_mut(*index - 1) {
        if prev_token.kind == TokenKind::Plain {
          prev_token.content.push_str(&token.content);
        }
      } else {
        tokenized.tokens.insert(*index, token);
      }
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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__escape_open_italic() {
    let input = "\\*not italic*";
    let expected = [
      Token{ kind: TokenKind::EscapedChar, content: "*".to_string(), position: Position { line: 0, column: 1 } },
      Token{ kind: TokenKind::Plain, content: "not".to_string(), position: Position { line: 0, column: 2 } },
      Token{ kind: TokenKind::Space, content: " ".to_string(), position: Position { line: 0, column: 5 } },
      Token{ kind: TokenKind::Plain, content: "italic*".to_string(), position: Position { line: 0, column: 6 } },
    ];

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }

  #[test]
  pub fn test_formatting__asterisks_as_plain() {
    let input = "before****after";
    let expected = [
      Token{ kind: TokenKind::Plain, content: "before****after".to_string(), position: Position { line: 0, column: 0 } },
    ];

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

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

    let actual = input.tokenize();

    assert_eq!(actual, expected, "{}", EXPECTED_MSG);
  }
}

