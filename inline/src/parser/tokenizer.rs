use std::collections::{HashMap, hash_map::Entry::Vacant};

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
        if last.kind == TokenKind::Space || last.kind == TokenKind::NewLine {
          enforce_open_constraint(&mut tokenized, last_token_index);
        }
        update_open_map(&mut tokenized, last_token_index);
        handle_last_closing_token(&mut tokenized);
        tokenized.tokens.push(last);

        last_token_index = updated_last_token_index;
      }
    }

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
      SingleTokenKind::Backslash => { tokenized.escape_active = true; },
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
    match last.kind {
        TokenKind::BoldClose => { open_index = tokenized.open_tokens.remove(&TokenKind::BoldOpen).unwrap(); },
        TokenKind::ItalicClose => { open_index = tokenized.open_tokens.remove(&TokenKind::ItalicOpen).unwrap(); },
        TokenKind::BoldItalicClose => { open_index = tokenized.open_tokens.remove(&TokenKind::BoldItalicOpen).unwrap(); },
        TokenKind::VerbatimClose => { open_index = tokenized.open_tokens.remove(&TokenKind::VerbatimOpen).unwrap(); },
        TokenKind::EmojiClose => { open_index = tokenized.open_tokens.remove(&TokenKind::EmojiOpen).unwrap(); },
        TokenKind::CommentClose => { open_index = tokenized.open_tokens.remove(&TokenKind::CommentOpen).unwrap(); },
        _ => { return; },
    }

    let mut updated_open_tokens = HashMap::new();
    for (kind, index) in &tokenized.open_tokens {
      if *index < open_index {
        updated_open_tokens.insert(*kind, *index);
      }
    }
    tokenized.open_tokens = updated_open_tokens;
  }
}

fn update_open_map(tokenized: &mut Tokenized, last_token_index: usize) {
  if let Some(last) = tokenized.tokens.last_mut() {
    // Note: Makes sure that no two open tokens of the same kind are before one closing one
    if let Vacant(e) = tokenized.open_tokens.entry(last.kind) {
      match last.kind {
        TokenKind::BoldOpen
        | TokenKind::ItalicOpen
        | TokenKind::BoldItalicOpen
        | TokenKind::VerbatimOpen
        | TokenKind::EmojiOpen
        | TokenKind::CommentOpen => { e.insert(last_token_index); },
        _ => {  },
      }
    } else {
      last.kind = TokenKind::Plain;
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

/// Function to enforce that open tokens do not allow space directly after it, except verbatim
fn enforce_open_constraint(tokenized: &mut Tokenized, last_token_index: usize) {
  if let Some(last) = tokenized.tokens.last_mut() {
    if last.kind != TokenKind::VerbatimOpen {
      if let Some(open_token_index) = tokenized.open_tokens.remove(&last.kind) {
        // invalid open token detected
        if open_token_index == last_token_index {
          last.kind = TokenKind::Plain;
        } else {
          tokenized.open_tokens.insert(last.kind, open_token_index);
        }
      }
    }
  }
}

fn update_asterisk(tokenized: &mut Tokenized, c: char) {
  match tokenized.tokens.pop() {
    Some(mut last) => {
      if last.kind == TokenKind::ItalicOpen {
        last.content.push(c);

        if let Some(bold_open) = tokenized.open_tokens.remove(&TokenKind::BoldOpen) {
          let preceding_token = tokenized.tokens.last().expect("Tokens must not be empty, because open token exists");
          if preceding_token.kind == TokenKind::Space || preceding_token.kind == TokenKind::NewLine {
            // Close after space is not allowed
            last.kind = TokenKind::Plain;
            tokenized.open_tokens.insert(TokenKind::BoldOpen, bold_open);
          } else {
            last.kind = TokenKind::BoldClose;
          }
        } else {
          last.kind = TokenKind::BoldOpen;
        }
        tokenized.tokens.push(last);    
      } else if last.kind == TokenKind::BoldOpen {
        if let Some(italic_open) = tokenized.open_tokens.get(&TokenKind::ItalicOpen) {
          // Note: handles cases like `*italic***bold**`
          let preceding_token = tokenized.tokens.last().expect("Tokens must not be empty, because open token exists");
          if preceding_token.kind == TokenKind::Space || preceding_token.kind == TokenKind::NewLine {
            // If Space is before `***`, it is split into [plain|italicClose|italicOpen] -> `*before ***after*` = `[io]before*[ic][io]after[ic]
            last.kind = TokenKind::Plain;
            last.content = TokenKind::ItalicOpen.as_str().to_string();
            tokenized.cur_pos.column += last.length();
            tokenized.tokens.push(last);
            let italic_close_token = Token { kind: TokenKind::ItalicClose, content: TokenKind::ItalicClose.as_str().to_string(), position: tokenized.cur_pos };
            tokenized.cur_pos.column += italic_close_token.length();
            tokenized.tokens.push(italic_close_token);
  
            handle_last_closing_token(tokenized);
  
            let italic_open_token = Token { kind: TokenKind::ItalicOpen, content: TokenKind::ItalicClose.as_str().to_string(), position: tokenized.cur_pos };
            tokenized.tokens.push(italic_open_token);
          } else {
            last.kind = TokenKind::ItalicClose;
            last.content = TokenKind::ItalicClose.as_str().to_string();
            tokenized.cur_pos.column += last.length();
            tokenized.tokens.push(last);
            
            handle_last_closing_token(tokenized);
  
            let bold_open_token = Token { kind: TokenKind::BoldOpen, content: TokenKind::BoldOpen.as_str().to_string(), position: tokenized.cur_pos };
            tokenized.tokens.push(bold_open_token);
          }  
        } else {
          last.kind = TokenKind::BoldItalicOpen;
          last.content.push(c);
          tokenized.tokens.push(last);
        }
      } else if last.kind == TokenKind::BoldItalicOpen {
        // Note: handles `****` as empty bold
        last.kind = TokenKind::BoldOpen;
        tokenized.cur_pos.column += last.length();
        tokenized.tokens.push(last);
        let new_token = Token{ kind: TokenKind::BoldClose, content: c.to_string(), position: tokenized.cur_pos };
        tokenized.tokens.push(new_token);
      } else if last.kind == TokenKind::ItalicClose {
        last.content.push(c);

        if tokenized.open_tokens.contains_key(&TokenKind::BoldOpen) || tokenized.open_tokens.contains_key(&TokenKind::BoldItalicOpen) {
          last.kind = TokenKind::BoldClose;
        } else {
          last.kind = TokenKind::BoldOpen;
        }

        tokenized.tokens.push(last);
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
        if tokenized.open_tokens.contains_key(&TokenKind::ItalicOpen) {
          if last.kind == TokenKind::Space || last.kind == TokenKind::NewLine {
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


fn cleanup_loose_open_tokens(tokenized: &mut Tokenized) {
  // all remaining hashmap entries get turned into plain tokens
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

}

