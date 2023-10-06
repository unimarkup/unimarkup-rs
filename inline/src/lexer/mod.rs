mod resolver;
mod token;

pub use token::*;
use unimarkup_commons::scanner::{
    position,
    span::{Span, SpanLen},
    Symbol, SymbolKind,
};

use crate::{Substitute, Substitutor};

use self::resolver::{RawToken, TokenResolver};

/// Used to create a Unimarkup [`Lexer`] over some data structure, most typically over some kind of
/// string, i.e. [`&str`].
///
/// [`Lexer`]: self::Lexer
/// [`&str`]: &str
pub trait Tokenize<'input> {
    /// Returns tokens found in self.
    ///
    /// # Lifetimes
    ///
    /// - `'input` - Lifetime of the input instance.
    fn tokens(&'input self) -> Tokens<'input>;
}

impl<'symbol, T> Tokenize<'symbol> for T
where
    T: AsRef<[Symbol<'symbol>]>,
{
    fn tokens(&'symbol self) -> Tokens<'symbol> {
        let lexer = Lexer {
            input: self.as_ref(),
        };

        Tokens::new(lexer.resolved())
    }
}

/// Lexer of Unimarkup inline formatted text. Generates a stream of [`Token`]s from input.
///
/// # Lifetimes
///
/// - `'input` - Lifetime of the input instance the lexer is lexing over.
///
/// [`Token`]: self::token::Token
pub struct Lexer<'input> {
    input: &'input [Symbol<'input>],
}

pub(crate) trait SymbolExt {
    /// Returns the [`LexLength`] a given symbol may have.
    ///
    /// [`LexLength`]: self::LexLength
    fn allowed_len(&self) -> LexLength;

    /// Returns the (UTF-8) length of symbol.
    fn len(&self) -> usize;

    /// Checks whether the grapheme is some Unimarkup Inline symbol.
    /// e.g. "*" can be start of Unimarkup Italic or Bold.
    fn is_keyword(&self) -> bool;

    /// Checks whether the given grapheme could be a start of an inline substitution.
    fn is_start_of_subst(&self, substitutor: &Substitutor) -> bool;

    /// Checks whether the grapheme is "\".
    fn is_esc(&self) -> bool;

    /// Checks whether the grapheme is any of the whitespace characters.
    fn is_whitespace(&self) -> bool;

    /// Checks whether the grapheme is a valid newline symbol.
    fn is_newline(&self) -> bool;

    /// Checks whether the grapheme has any significance in escape sequence.
    /// e.g. The lexer interprets "\ " as a Whitespace `Token`
    fn is_significant_esc(&self) -> bool;
}

impl SymbolExt for Symbol<'_> {
    fn allowed_len(&self) -> LexLength {
        match self.kind {
            SymbolKind::Star | SymbolKind::Underline => LexLength::Limited(3),

            SymbolKind::Backslash
            | SymbolKind::Caret
            | SymbolKind::Overline
            | SymbolKind::Tick
            | SymbolKind::Dollar => LexLength::Limited(1),

            SymbolKind::OpenParenthesis
            | SymbolKind::CloseParenthesis
            | SymbolKind::OpenBracket
            | SymbolKind::CloseBracket
            | SymbolKind::OpenBrace
            | SymbolKind::CloseBrace => LexLength::Exact(1),

            SymbolKind::Pipe | SymbolKind::Tilde | SymbolKind::Quote | SymbolKind::Colon => {
                LexLength::Limited(2)
            }

            SymbolKind::Whitespace | SymbolKind::Newline | SymbolKind::Plain => {
                LexLength::Unlimited
            }

            // Symbols not part of inline syntax are treated as plain text
            _ => LexLength::Unlimited,
        }
    }

    fn len(&self) -> usize {
        self.as_str().len()
    }

    fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            SymbolKind::Star
                | SymbolKind::Underline
                | SymbolKind::Caret
                | SymbolKind::Tick
                | SymbolKind::Overline
                | SymbolKind::Pipe
                | SymbolKind::Tilde
                | SymbolKind::Quote
                | SymbolKind::Dollar
                | SymbolKind::OpenParenthesis
                | SymbolKind::CloseParenthesis
                | SymbolKind::OpenBracket
                | SymbolKind::CloseBracket
                | SymbolKind::OpenBrace
                | SymbolKind::CloseBrace
        )
    }

    fn is_start_of_subst(&self, substitutor: &Substitutor) -> bool {
        substitutor.is_start_of_substitute(self)
    }

    fn is_esc(&self) -> bool {
        matches!(self.kind, SymbolKind::Backslash)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self.kind, SymbolKind::Whitespace)
    }

    fn is_newline(&self) -> bool {
        matches!(self.kind, SymbolKind::Newline)
    }

    fn is_significant_esc(&self) -> bool {
        self.is_whitespace() || self.is_newline()
    }
}

impl<'token> Lexer<'token> {
    /// Creates a [`TokenIterator`] from [`Lexer`].
    ///
    /// [`TokenIterator`]: self::TokenIterator
    /// [`Lexer`]: self::Lexer
    fn iter(&self) -> TokenIterator<'token> {
        let symbols = self.input;

        TokenIterator {
            symbols,
            index: 0,
            substitutor: Substitutor::new(),
        }
    }

    /// Creates a [`TokenResolver`] from [`Lexer`].
    ///
    /// [`TokenResolver`]: self::resolver::TokenResolver
    /// [`Lexer`]: self::Lexer
    fn resolved(self) -> TokenResolver<'token> {
        TokenResolver::new(self.iter())
    }
}

impl<'input> IntoIterator for &'input Lexer<'input> {
    type Item = Token<'input>;

    type IntoIter = TokenIterator<'input>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Enum used for annotating whether the literal content for some [`Symbol`] should be stored into [`Token`] or not.
///
/// [`Symbol`]: self::Symbol
/// [`Token`]: self::token::Token
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ContentOption {
    /// Annotates that content should be stored into [`Token`].
    ///
    /// [`Token`]: crate::Token
    Store,

    /// Annotates that content should **NOT** be stored into [`Token`].
    ///
    /// [`Token`]: crate::Token
    Discard,
}

/// Helper enum for annotation of allowed length for some given [`Symbol`]
///
/// [`Symbol`]: self::Symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LexLength {
    /// Any length allowed.
    Unlimited,
    /// Exact length allowed.
    Exact(usize),
    /// Any length up to the limit.
    Limited(usize),
}

impl From<usize> for LexLength {
    fn from(len: usize) -> Self {
        Self::Exact(len)
    }
}

/// Iterator over Unimarkup [`Token`]s, performs the actual lexing.
///
/// # Lifetimes
///
/// * `'input` - Lifetime of the input string, lives at least as long as this instance of
/// `TokenIterator`.
///
/// [`Token`]: self::token::Token
#[derive(Debug, Clone)]
pub struct TokenIterator<'input> {
    /// Pool of available symbols found in the input.
    symbols: &'input [Symbol<'input>],

    /// Index used as a cursor to the symbol we are currently interpreting.
    index: usize,

    /// [`Substitutor`] used for resolving inline substitutions. Right now, substitutor uses only
    /// built-in substitutions and has 'static lifetime per default, and can be shortened to any
    /// other lifetime.
    substitutor: Substitutor<'input>,
}

impl<'input> TokenIterator<'input> {
    /// Returns the [`Symbol`] at the given index, from the underlying pool of symbols.
    ///
    /// # Lifetimes
    ///
    /// - `'input` - Lifetime of the [`Symbol`]s, tied to the input symbol is found in and lives at
    /// least as long as this instance of `TokenIterator`.
    fn get_symbol(&self, index: usize) -> Option<&'input Symbol<'input>> {
        self.symbols.get(index)
    }

    /// Lexes given [`Symbol`]s with significance, i.e. `**` and produces a [`Token`] out of it,
    /// if possible.
    ///
    /// # Lifetimes
    ///
    /// - `'input` - Lifetime of the input the token is found in and lives at least as long as
    /// this instance of `TokenIterator`.
    ///
    /// [`Token`]: self::token::Token
    /// [`Symbol`]: self::Symbol
    fn lex_keyword(&mut self) -> Option<Token<'input>> {
        // NOTE: General invariant of lexing:
        // If some contiguous symbol occurrence exceeds the maximal symbol length, the contiguous
        // sequence is lexed as plain (e.g. ****).

        let symbol = self.get_symbol(self.index)?;

        let subst = self.get_substitute(symbol);

        let start_pos = symbol.start;
        let end_pos = subst
            .as_ref()
            .map_or_else(|| self.symbol_len(symbol), |subst| subst.span().end);

        let span = Span::from((start_pos, end_pos));
        let symbol_len = span.len_grapheme()?;
        let spacing = self.spacing_around(symbol_len);

        let kind = subst.as_ref().map_or_else(
            || TokenKind::from((symbol, symbol_len)),
            |_| TokenKind::Plain,
        );

        let curr_index = self.index + symbol_len;
        let content = subst.map_or_else(
            || Symbol::flatten(&self.symbols[self.index..curr_index]).unwrap(),
            |subst| subst.as_str(),
        );

        let token = Token::with_conditional_content(
            kind,
            Span::from((start_pos, end_pos)),
            spacing,
            content,
            kind.content_option(),
        );

        self.index = curr_index;

        Some(token)
    }

    /// Returns the lexed length of a given [`Symbol`] based on the allowed [`LexLength`]
    /// constraint of the [`Symbol`].
    ///
    /// For [`Symbol`] with exact [`LexLength::Exact`], the length returned is equal or smaller
    /// than that exact length. This behavior is used to temporarily disable lexing invariant.
    ///
    /// The invariant in general is that, for any given [`Symbol`], scanning more symbols than
    /// it's expected to produce a valid [`Token`], the [`TokenKind`] is changed to
    /// [`TokenKind::Plain`] no matter what the first [`Symbol`] implies.
    ///
    /// For example:
    /// - `*` is seen as one [`Symbol::Star`] literals, and is lexed as [`TokenKind::Italic`]
    /// - `**` is seen as two [`Symbol::Star`] literals, and is lexed as [`TokenKind::Bold`]
    /// - `***` is seen as three [`Symbol::Star`] literals, and is lexed as [`TokenKind::ItalicBold`]
    /// - `****` is seen as four [`Symbol::Star`] literals, so it's more than expected and is lexed
    /// as [`TokenKind::Plain`].
    ///
    /// Disabling the invariant is necessary for some [`Token`]s where we want to stop further
    /// scanning as soon as one valid [`Token`] is lexed. That is the case for [`Symbol::OpenBracket`].
    /// Consecutive `[` literals are seen as distinct starts of a text group inline format.
    ///
    /// [`Symbol`]: self::Symbol
    /// [`Symbol::Star`]: self::Symbol::Star
    /// [`Symbol::OpenBracket`]: self::Symbol::OpenBracket
    /// [`LexLength`]: self::LexLength
    /// [`LexLength::Exact`]: self::LexLength::Exact
    /// [`Token`]: self::token::Token
    /// [`TokenKind`]: self::token::TokenKind
    /// [`TokenKind::Plain`]: self::token::TokenKind::Plain
    /// [`TokenKind::Italic`]: self::token::TokenKind::Italic
    /// [`TokenKind::Bold`]: self::token::TokenKind::Bold
    /// [`TokenKind::ItalicBold`]: self::token::TokenKind::ItalicBold
    fn symbol_len(&self, symbol: &Symbol) -> position::Position {
        let end_index = self.literal_end_index(symbol);
        let scanned_len = end_index - self.index;

        let idx = match symbol.allowed_len() {
            LexLength::Exact(len) => self.index + scanned_len.min(len),
            _ => end_index,
        };

        self.get_symbol(idx)
            .expect("Symbol already seen, must exist")
            .end
    }

    /// Finds the furthest grapheme in line where, starting from the current cursor position,
    /// each grapheme matches the one provided as the `symbol`.
    ///
    /// Note that the current cursor position will be returned if the grapheme under cursor doesn't
    /// match the `symbol` grapheme provided as the function parameter.
    fn literal_end_index(&self, symbol: &Symbol) -> usize {
        let mut pos = self.index;

        loop {
            match self.get_symbol(pos) {
                Some(sym) if sym.kind == symbol.kind => pos += 1,
                _ => break pos - 1,
            }
        }
    }

    /// Calculates the [`Spacing`] just before the cursor position and after cursor position and
    /// the given len.
    ///
    /// [`Spacing`]: self::token::Spacing
    fn spacing_around(&self, len: usize) -> Spacing {
        let mut spacing = Spacing::None;

        if self.is_whitespace_at_offs(-1) {
            spacing += Spacing::Pre;
        }
        if self.is_whitespace_at_offs(len as isize) {
            spacing += Spacing::Post;
        }

        spacing
    }

    /// Check if character at cursor position with offset is whitespace.
    fn is_whitespace_at_offs(&self, offset: isize) -> bool {
        let pos = if offset < 0 {
            let offset = offset.unsigned_abs();

            match offset.saturating_sub(self.index) {
                1 => return true, // NOTE: right before begin of line counts as whitespace
                2.. => return false,
                _ => self.index.saturating_sub(offset),
            }
        } else {
            match self.index.saturating_add(offset as usize) {
                // NOTE: end of line symbol IS whitespace
                pos if pos == self.symbols.len() => return true,
                pos if pos > self.symbols.len() => return false,
                pos => pos,
            }
        };

        self.get_symbol(pos).map_or(true, |ch| ch.is_whitespace())
    }

    /// Lexes a [`Token`] with [`TokenKind::Plain`], [`TokenKind::Whitespace`] or
    /// [`TokenKind::Newline`], with content being original Unimarkup input for the given token.
    ///
    /// # Lifetimes
    ///
    /// - `'input` - Lifetime of the input the token is found in and lives at least as long as
    /// this instance of `TokenIterator`.
    ///
    /// [`Token`]: self::token::Token
    /// [`TokenKind::Plain`]: self::token::TokenKind::Plain
    /// [`TokenKind::Whitespace`]: self::token::TokenKind::Whitespace
    /// [`TokenKind::Newline`]: self::token::TokenKind::Newline
    fn lex_plain(&mut self) -> Option<Token<'input>> {
        // Token is not a keyword, but can be one of multiple plain tokens:
        // 1. Newline found -> Newline token
        // 2. Whitespace found -> Whitespace token
        // 3. any other grapheme -> Plain token

        let start_sym = self.get_symbol(self.index)?;
        let mut first = self.index;

        let sym = if start_sym.is_esc() {
            first += 1;
            self.get_symbol(first)?
        } else {
            start_sym
        };

        let kind = match sym.kind {
            SymbolKind::Whitespace => TokenKind::Whitespace,
            SymbolKind::Newline => TokenKind::Newline,
            _ => TokenKind::Plain,
        };

        let first_sym = sym;
        let mut last = first;
        self.index += 1;

        if kind == TokenKind::Plain {
            while let Some(sym) = self.get_symbol(self.index) {
                if sym.kind == first_sym.kind {
                    last = self.index;
                    self.index += 1;
                } else {
                    break;
                }
            }
        }

        // NOTE: index points to the NEXT character, token Span is UP TO that character
        let start_pos = start_sym.start;
        let end_pos = self.get_symbol(last)?.end;

        let span = Span::from((start_pos, end_pos));
        let content = Symbol::flatten(&self.symbols[first..=last]);

        let len = span.len_grapheme().unwrap_or(1);
        let spacing = self.spacing_around(len);

        let token = Token {
            kind,
            span,
            spacing,
            content,
        };

        Some(token)
    }

    /// Lexes an escaped [`Symbol`], creating [`Token`] with either [`TokenKind::Plain`] or some
    /// kind of significant escape, such es escaped newline.
    ///
    /// # Lifetimes
    ///
    /// - `'input` - Lifetime of the input the token is found in and lives at least as long as
    /// this instance of `TokenIterator`.
    ///
    /// [`Symbol`]: self::Symbol
    /// [`Token`]: self::token::Token
    /// [`TokenKind::Plain`]: self::token::TokenKind::Plain
    fn lex_escape_seq(&mut self) -> Option<Token<'input>> {
        let i = self.index;
        let symbol = self.get_symbol(i)?;

        // NOTE: index here is pointing to the current grapheme
        let start_pos = self.get_symbol(self.index)?.start; // escape character
        let end_pos = start_pos + SpanLen::from(symbol.len());

        let token_kind = if symbol.is_whitespace() {
            TokenKind::EscapedWhitespace
        } else {
            TokenKind::EscapedNewline
        };

        let token = Token {
            kind: token_kind,
            span: Span::from((start_pos, end_pos)),
            spacing: Spacing::None,
            content: Some(symbol.as_str()),
        };

        self.index += 1;
        Some(token)
    }

    /// Returns a [`Substitute`] if a builtin or registered [`Substitute`] is found for a given
    /// [`Symbol`] or list of [`Symbol`]s.
    ///
    /// # Lifetimes
    ///
    /// - `'input` - Lifetime of the input the [`Symbol`] is found in and lives at least as long as
    /// this instance of `TokenIterator`.
    ///
    /// [`Symbol`]: self::Symbol
    /// [`Substitute`]: crate::inlines::Substitute
    fn get_substitute(&self, symbol: &Symbol<'input>) -> Option<Substitute<'input>> {
        if !self.substitutor.is_start_of_substitute(symbol) {
            return None;
        }

        let iter = {
            self.symbols[self.index..]
                .iter()
                .take(self.substitutor.max_len())
                .take_while(|symbol| !symbol.is_whitespace())
        };

        if let Spacing::Both = self.spacing_around(iter.clone().count()) {
            self.substitutor.get_subst_iter(iter)
        } else {
            None
        }
    }
}

impl<'input> Iterator for TokenIterator<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        // three cases:
        // 1. next grapheme is keyword -> generate some token
        // 2. next grapheme is '\' -> handle escape sequence
        // 3. next grapheme is not a keyword -> it is plain text

        match self.get_symbol(self.index) {
            Some(symbol) if symbol.is_keyword() || symbol.is_start_of_subst(&self.substitutor) => {
                self.lex_keyword()
            }
            Some(symbol) if symbol.is_esc() => {
                // Three cases:
                // 1. next character has significance in escape sequence -> some token
                // 2. next character has no significance -> lex as plain text
                // 3. there is no next character. That implies that we've got to end of input.

                let sym = self.get_symbol(self.index)?;
                if sym.is_significant_esc() {
                    self.index += 1;
                    self.lex_escape_seq()
                } else {
                    self.lex_plain()
                }
            }
            _ => self.lex_plain(),
        }
    }
}

/// Iterator over the Unimarkup Inline [`Token`]s found in the given input. This iterator is
/// available for any type that implements [`Tokenize`] trait. The [`Tokenize`] trait can be
/// implemented directly, or automatically if type implements [`AsRef<Symbol>`] trait.
///
/// Iterator returns resolved [`Token`]s, meaning that if [`Token`] is marked as an opening
/// [`Token`] for some inline, it is guaranteed that the closing [`Token`] will be returned at a
/// later point (and vice versa).
///
/// # Lifetimes
///
/// * `'input` - Lifetime of the input the [`Token`]s are lexed from.
#[derive(Debug, Clone)]
pub struct Tokens<'input> {
    iter: resolver::IntoIter<'input>,
    cache: Option<RawToken<'input>>,
}

impl<'input> Tokens<'input> {
    pub(crate) fn new(resolver: TokenResolver<'input>) -> Self {
        Self {
            iter: resolver.into_iter(),
            cache: None,
        }
    }
}

impl<'input> Iterator for Tokens<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut unr_token = if let Some(unr_token) = self.cache.take() {
            unr_token
        } else {
            self.iter.next()?
        };

        match unr_token.pop() {
            Some(first_part) => {
                // save remaining part
                self.cache = Some(unr_token);
                Some(Token::from(first_part))
            }
            _ => Some(Token::from(unr_token)),
        }
    }
}
