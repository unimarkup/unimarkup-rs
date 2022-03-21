use unicode_segmentation::UnicodeSegmentation;

use crate::Position;

pub type Tokens = Vec<Token>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub content: String,
  pub position: Position,
}

impl Token {
  pub fn length(&self) -> usize {
    if self.kind == TokenKind::NewLine {
      return 0;
    }
    self.content.graphemes(true).count()
  }

  pub fn is_space_or_newline(&self) -> bool {
    self.kind.is_space_or_newline()
  }

  pub fn closes_scope(&self) -> bool {
    self.kind == TokenKind::TextGroupClose
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SingleTokenKind {
  Plain,
  Newline,
  Space,
  Backslash,
  ExclamationMark,
  Ampersand,
  Colon,
  Caret,
  Underscore,
  Asterisk,
  Plus,
  Accent,
  LeftSquareBracket,
  RightSquareBracket,
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
  EscapedGrapheme,
  NewLine,
  Space,
  CommentOpen,
  CommentClose,
  DirectUnicode,
  TextGroupOpen,
  TextGroupClose,
  Eoi,
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
      TokenKind::TextGroupOpen => "[",
      TokenKind::TextGroupClose => "]",

      // Note: Below are only placeholder valus
      TokenKind::Plain => "",
      TokenKind::EscapedGrapheme => "\\",
      TokenKind::NewLine => "\n",
      TokenKind::Space => " ",
      TokenKind::DirectUnicode => "&U+1F816;",
      TokenKind::Eoi => "",
    }
  }

  pub fn is_space_or_newline(&self) -> bool {
    self == &TokenKind::Space || self == &TokenKind::NewLine
  }
}


pub trait AsSingleTokenKind {
  fn as_single_token_kind(&self) -> SingleTokenKind;
}

impl AsSingleTokenKind for &str {
    fn as_single_token_kind(&self) -> SingleTokenKind {
      match *self {
        "*" => { SingleTokenKind::Asterisk },
        "\\" => { SingleTokenKind::Backslash },
        "`" => { SingleTokenKind::Accent },
        "[" => { SingleTokenKind::LeftSquareBracket },
        "]" => { SingleTokenKind::RightSquareBracket },
        grapheme => {
          if grapheme.is_newline() {
            return SingleTokenKind::Newline;
          } else if grapheme.trim().is_empty() {
            return SingleTokenKind::Space;
          }
          SingleTokenKind::Plain
        }
      }
    }
}

pub trait Newline {
  fn is_newline(&self) -> bool;
}

impl Newline for &str {
  /// Note: Only temporary solution until rust supports is_newline() per default.
  /// 
  /// Treats `\n`, `\r\n` and `\r` as one newline.
  fn is_newline(&self) -> bool {
    let s = *self;    
    s == "\n" || s == "\r\n" || s == "\r"
  }
}

impl Newline for String {
  /// Note: Only temporary solution until rust supports is_newline() per default.
  /// 
  /// Treats `\n`, `\r\n` and `\r` as one newline.
  fn is_newline(&self) -> bool {
    self.as_str().is_newline()
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

