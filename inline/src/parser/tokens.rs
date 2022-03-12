use crate::Position;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub content: String,
  pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  BoldOpen,
  BoldClose,
  ItalicOpen,
  ItalicClose,
  BoldItalicOpen,
  BoldItalicClose,
  Plain,
  EmojiOpen,
  EmojiClose,
  PossibleDirectEmoji,
  PossibleDirectArrow,
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
  fn as_str(&self) -> &'static str {
    match *self {
      TokenKind::BoldOpen => "**",
      TokenKind::BoldClose => TokenKind::BoldOpen.as_str(),
      TokenKind::ItalicOpen => "*",
      TokenKind::ItalicClose => TokenKind::ItalicOpen.as_str(),
      TokenKind::BoldItalicOpen => "***",
      TokenKind::BoldItalicClose => TokenKind::BoldItalicOpen.as_str(),
      TokenKind::EmojiOpen => "::",
      TokenKind::EmojiClose => TokenKind::EmojiOpen.as_str(),  
      TokenKind::CommentOpen => ";;",
      TokenKind::CommentClose => TokenKind::CommentOpen.as_str(),

      // Note: Below are only placeholder valus
      TokenKind::Plain => "",
      TokenKind::PossibleDirectEmoji => ":D",
      TokenKind::PossibleDirectArrow => "-->",
      TokenKind::EscapedChar => "\\",
      TokenKind::NewLine => "\n",
      TokenKind::Space => " ",
      TokenKind::DirectUnicode => "&U+1F816;"
    }
  }
}

pub trait Keyword {
  fn is_keyword(&self) -> Option<TokenKind>;
  fn is_newline(&self) -> bool;
}

impl Keyword for char {
    fn is_keyword(&self) -> Option<TokenKind> {
      let s = self.to_string();
      return s.as_str().is_keyword();
    }

    fn is_newline(&self) -> bool {
      let s = self.to_string();
      return s.as_str().is_newline();
    }
}

impl Keyword for &str {
    fn is_keyword(&self) -> Option<TokenKind> {
      let s = *self;

      // Note: Close token is omitted if open and close are equal
      if s == TokenKind::BoldOpen.as_str() {
        return Some(TokenKind::BoldOpen);
      } else if s == TokenKind::ItalicOpen.as_str() {
        return Some(TokenKind::ItalicOpen);
      } else if s == TokenKind::BoldItalicOpen.as_str() {
        return Some(TokenKind::BoldItalicOpen);
      } else if s == TokenKind::EmojiOpen.as_str() {
        return Some(TokenKind::EmojiOpen);
      } else if let Some(arrow) = possible_arrow(s) {
        return Some(arrow);
      } else if let Some(emoji) = possible_emoji(s) {
        return Some(emoji);
      } else if let Some(direct_unicode) = possible_direct_unicode(s) {
        return Some(direct_unicode);
      }
    
      None
    }

    fn is_newline(&self) -> bool {
      let s = *self;
      //Note: Only temporary solution until rust supports is_newline() per default
      s == "\n" || s == "\r\n" || s == "\r"
    }
}

pub fn possible_arrow(s: &str) -> Option<TokenKind> {
  if s.contains(|c| c == '-' || c == '=' || c == '<' || c == '>' || c == '|') {
    return Some(TokenKind::PossibleDirectArrow);
  }
  None
}

pub fn possible_emoji(s: &str) -> Option<TokenKind> {
  if s.contains(|c| c == '-' || c == '=' || c == '<' || c == '>' || c == ')'
    || c == '(' || c == '^' || c == 'O' || c == 'D' || c == 'Y' || c == 'N' || c == 'P'
    || c == '3' || c == '/' || c == ':' || c == ';' || c == '_') {

    return Some(TokenKind::PossibleDirectEmoji);
  }
  None
}

pub fn possible_direct_unicode(s: &str) -> Option<TokenKind> {
  if s.contains(|c: char| c == '&' || c == 'U' || c == '+' || c.is_digit(16) || c == ';') {
    return Some(TokenKind::DirectUnicode)
  }
  None
}

