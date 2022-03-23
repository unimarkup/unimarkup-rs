//! Defines all tokens used for tokenization.

use unicode_segmentation::UnicodeSegmentation;

use super::Position;

/// Type representing a list of tokens
pub type Tokens = Vec<Token>;

/// Token structure representing all supported inline elements with their
/// content and position inside a given input.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Token {
  /// The token kind identifies the token parts of an Unimarkup inline element
  pub kind: TokenKind,
  /// The content of the token
  pub content: String,
  /// The starting position of this token inside a given input
  pub position: Position,
}

impl Token {
  /// Returns the content length of a token.
  /// The length is the number of Unicode graphemes inside the content.
  pub fn length(&self) -> usize {
    if self.kind == TokenKind::NewLine {
      return 0;
    }
    self.content.graphemes(true).count()
  }

  /// Shows if a token is of kind space or newline.
  pub fn is_space_or_newline(&self) -> bool {
    self.kind.is_space_or_newline()
  }

  /// Shows if a token is closing a scope inside a given input.
  /// Closing scopes may be closing text groups, closing attribute blocks, ...
  pub fn closes_scope(&self) -> bool {
    self.kind == TokenKind::TextGroupClose
  }
}

/// Enum defining all special single graphemes understood by Unimarkup. 
#[derive(Debug, Clone, PartialEq)]
pub enum SingleTokenKind {
  /// Default kind for all non-special graphemes.
  Plain,
  /// Represents a newline grapheme.
  Newline,
  /// Represents a grapheme that has the Unicode whitespace property and is not a newline.
  Space,
  /// Represents `\`.
  Backslash,
  // ExclamationMark,
  // Ampersand,
  // Colon,
  // Caret,
  // Underscore,
  /// Represents `*`.
  Asterisk,
  // Plus,
  /// Represents `` ` ``.
  Accent,
  /// Represents `[`.
  LeftSquareBracket,
  /// Represents `]`.
  RightSquareBracket,
}

/// Enum representing tokens that are part of Unimarkup inline elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TokenKind {
  /// Represents the open part of bold inline formatting.
  BoldOpen,
  /// Represents the closing part of bold inline formatting.
  BoldClose,
  /// Represents the open part of italic inline formatting.
  ItalicOpen,
  /// Represents the closing part of italic inline formatting.
  ItalicClose,
  /// Represents the combined open part of bold and italic inline formatting.
  BoldItalicOpen,
  /// Represents the combined closing part of bold and italic inline formatting.
  BoldItalicClose,
  /// Represents the open part of verbatim inline formatting.
  VerbatimOpen,
  /// Represents the closing part of verbatim inline formatting.
  VerbatimClose,
  /// Represents a plain text part.
  Plain,
  /// Represents the open part of an inline emoji shortcut.
  EmojiOpen,
  // EmojiClose,
  /// Represents a grapheme that is escaped by a backslash.
  EscapedGrapheme,
  /// Represents a newline as defined by `is_newline()`.
  NewLine,
  /// Represents a grapheme that has the Unicode whitespace property and is not a newline.
  Space,
  // CommentOpen,
  // CommentClose,
  // DirectUnicode,
  /// Represents the open part of an inline text group.
  TextGroupOpen,
  /// Represents the closing part of an inline text group.
  TextGroupClose,
  /// Represents the end of a given input.
  Eoi,
}

impl Default for TokenKind {
  /// Returns `Plain` as default token.
  fn default() -> Self {
    TokenKind::Plain
  }
}

impl TokenKind {
  /// Returns the string representation for a token.
  /// 
  /// e.g. `**` for BoldOpen and BoldClose.
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
      // TokenKind::EmojiClose => TokenKind::EmojiOpen.as_str(),  
      // TokenKind::CommentOpen => ";;",
      // TokenKind::CommentClose => TokenKind::CommentOpen.as_str(),
      TokenKind::TextGroupOpen => "[",
      TokenKind::TextGroupClose => "]",

      // Note: Below are only placeholder valus
      TokenKind::Plain => "",
      TokenKind::EscapedGrapheme => "\\",
      TokenKind::NewLine => "\n",
      TokenKind::Space => " ",
      // TokenKind::DirectUnicode => "&U+1F816;",
      TokenKind::Eoi => "",
    }
  }

  /// Shows if a token is either a space or newline.
  pub fn is_space_or_newline(&self) -> bool {
    self == &TokenKind::Space || self == &TokenKind::NewLine
  }
}

/// Trait to convert a type into a single token.
pub trait AsSingleTokenKind {
  /// Converts given type into a SingleTokenKind.
  /// 
  /// e.g. `*` --> `SingleTokenKind::Asterisk`
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
  /// Note: Only temporary solution until rust supports is_newline() per default.
  fn is_newline(&self) -> bool;
}

impl Newline for &str {
  /// Treats `\n`, `\r\n` and `\r` as one newline.
  fn is_newline(&self) -> bool {
    let s = *self;    
    s == "\n" || s == "\r\n" || s == "\r"
  }
}

impl Newline for String {
  fn is_newline(&self) -> bool {
    self.as_str().is_newline()
  }
}
