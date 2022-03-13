use unicode_segmentation::UnicodeSegmentation;

use crate::Position;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub content: String,
  pub position: Position,
}

impl Token {
  pub fn length(&self) -> usize {
    self.content.graphemes(true).count()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SingleTokenKind {
  Plain,
  LineFeed,
  CarriageReturn,
  Tab,
  Space,
  Backslash,
  ExclamationMark,
  Ampersand,
  Colon,
  Caret,
  Underscore,
  Asterisk,
  Plus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TokenKind {
  BoldOpen,
  BoldClose,
  ItalicOpen,
  ItalicClose,
  BoldItalicOpen,
  BoldItalicClose,
  VerbatimOpen,
  VerbatimClose,
  Plain,
  EmojiOpen,
  EmojiClose,
  DirectEmoji,
  DirectArrow,
  EscapedChar,
  NewLine,
  Space,
  CommentOpen,
  CommentClose,
  DirectUnicode,
}

impl Default for TokenKind {
    fn default() -> Self {
      TokenKind::Plain
    }
}

impl TokenKind {
  pub fn as_str(&self) -> &'static str {
    match *self {
      TokenKind::BoldOpen => "**",
      TokenKind::BoldClose => TokenKind::BoldOpen.as_str(),
      TokenKind::ItalicOpen => "*",
      TokenKind::ItalicClose => TokenKind::ItalicOpen.as_str(),
      TokenKind::BoldItalicOpen => "***",
      TokenKind::BoldItalicClose => TokenKind::BoldItalicOpen.as_str(),
      TokenKind::VerbatimOpen => "`",
      TokenKind::VerbatimClose => TokenKind::EmojiOpen.as_str(), 
      TokenKind::EmojiOpen => "::",
      TokenKind::EmojiClose => TokenKind::EmojiOpen.as_str(),  
      TokenKind::CommentOpen => ";;",
      TokenKind::CommentClose => TokenKind::CommentOpen.as_str(),

      // Note: Below are only placeholder valus
      TokenKind::Plain => "",
      TokenKind::DirectEmoji => ":D",
      TokenKind::DirectArrow => "-->",
      TokenKind::EscapedChar => "\\",
      TokenKind::NewLine => "\n",
      TokenKind::Space => " ",
      TokenKind::DirectUnicode => "&U+1F816;",
    }
  }
}


pub trait AsSingleTokenKind {
  fn as_single_token_kind(&self) -> SingleTokenKind;
}

impl AsSingleTokenKind for char {
    fn as_single_token_kind(&self) -> SingleTokenKind {
      match *self {
        '*' => { SingleTokenKind::Asterisk },

        c => {
          if c.is_whitespace() {
            return SingleTokenKind::Space;
          }
          SingleTokenKind::Plain
        }
      }
    }
}

pub trait AsTokenKind {
  fn as_token_kind(&self) -> TokenKind;
}

impl AsTokenKind for &str {
    fn as_token_kind(&self) -> TokenKind {
      let s = *self;

      // Note: Close token is omitted if open and close are equal
      if s == TokenKind::BoldOpen.as_str() {
        return TokenKind::BoldOpen;
      } else if s == TokenKind::ItalicOpen.as_str() {
        return TokenKind::ItalicOpen;
      } else if s == TokenKind::BoldItalicOpen.as_str() {
        return TokenKind::BoldItalicOpen;
      } else if s == TokenKind::EmojiOpen.as_str() {
        return TokenKind::EmojiOpen;
      }
    
      TokenKind::Plain
    }
}

pub trait Newline {
  fn is_newline(&self) -> bool;
}

impl Newline for &str {
  fn is_newline(&self) -> bool {
    let s = *self;
    //Note: Only temporary solution until rust supports is_newline() per default
    s == "\n" || s == "\r\n" || s == "\r"
  }
}

// pub fn possible_arrow(s: &str) -> Option<TokenKind> {
//   if s.contains(|c| c == '-' || c == '=' || c == '<' || c == '>' || c == '|') {
//     return Some(TokenKind::PossibleDirectArrow);
//   }
//   None
// }

// pub fn possible_emoji(s: &str) -> Option<TokenKind> {
//   if s.contains(|c| c == '-' || c == '=' || c == '<' || c == '>' || c == ')'
//     || c == '(' || c == '^' || c == 'O' || c == 'D' || c == 'Y' || c == 'N' || c == 'P'
//     || c == '3' || c == '/' || c == ':' || c == ';' || c == '_') {

//     return Some(TokenKind::PossibleDirectEmoji);
//   }
//   None
// }

// pub fn possible_direct_unicode(s: &str) -> Option<TokenKind> {
//   if s.contains(|c: char| c == '&' || c == 'U' || c == '+' || c.is_digit(16) || c == ';') {
//     return Some(TokenKind::DirectUnicode)
//   }
//   None
// }

