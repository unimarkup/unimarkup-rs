use itertools::Itertools;

use crate::{
    lexer::token::TokenKind,
    parsing::{Element, Parser},
};

use super::{
    token::{AttributeToken, AttributeTokenKind},
    tokenize::AttributeTokenizer,
};

macro_rules! at_rules {
    ($($name:literal -> $id:ident $(: $info:literal)?);*) => {

        /// At-rules that are supported as Unimarkup attributes.
        /// Taken from: https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule?retiredLocale=de
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum AtRuleId {
            $(
                #[doc=concat!("The `", $name, "` at-rule.")]
                $(
                    #[doc=concat!("See: ", $info)]
                )?
                $id,

            )*
        }

        impl TryFrom<&str> for AtRuleId {
            type Error = super::log_id::AttributeError;

            /// Tries to convert a given `str` to a [`AtRuleId`].
            /// The given `str` must **not** contain the leading `@`.
            ///
            /// Usage: `AtRuleId::try_from("media")`
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value.to_lowercase().as_str() {
                    $(
                        $name => Ok(AtRuleId::$id),
                    )*
                    _ => Err(super::log_id::AttributeError::UnsupportedAtRuleIdent),
                }
            }
        }

        impl AtRuleId{
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        AtRuleId::$id => $name,
                    )*
                }
            }

            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                match self {
                    $(
                        AtRuleId::$id => $name.len(),
                    )*
                }
            }
        }
    }
}

at_rules!(
    "media" -> Media: "https://developer.mozilla.org/en-US/docs/Web/CSS/@media";
    "container" -> Container: "https://developer.mozilla.org/en-US/docs/Web/CSS/@container";
    "layer" -> Layer: "https://developer.mozilla.org/en-US/docs/Web/CSS/@layer";
    "scope" -> Scope: "https://developer.mozilla.org/en-US/docs/Web/CSS/@scope";
    "keyframes" -> Keyframes: "https://developer.mozilla.org/en-US/docs/Web/CSS/@keyframes";
    "page" -> Page: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page";
    "top-left-corner" -> TopLeftCorner: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "top-left" -> TopLeft: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "top-center" -> TopCenter: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "top-right" -> TopRight: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "top-right-corner" -> TopRightCorner: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "bottom-left-corner" -> BottomLeftCorner: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "bottom-left" -> BottomLeft: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "bottom-center" -> BottomCenter: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "bottom-right" -> BottomRight: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "bottom-right-corner" -> BottomRightCorner: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "left-top" -> LeftTop: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "left-middle" -> LeftMiddle: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "left-bottom" -> LeftBottom: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "right-top" -> RightTop: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "right-middle" -> RightMiddle: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "right-bottom" -> RightBottom: "https://developer.mozilla.org/en-US/docs/Web/CSS/@page#margin_at-rules";
    "supports" -> Supports: "https://developer.mozilla.org/en-US/docs/Web/CSS/@supports";
    "counter-style" -> CounterStyle: "https://developer.mozilla.org/en-US/docs/Web/CSS/@counter-style";
    "starting-style" -> StartingStyle: "https://developer.mozilla.org/en-US/docs/Web/CSS/@starting-style";
    "color-profile" -> ColorProfile: "https://developer.mozilla.org/en-US/docs/Web/CSS/@color-profile";
    "font-face" -> FontFace: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-face";
    "font-feature-values" -> FontFeatureValues: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values";
    "swash" -> Swash: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#swash";
    "annotation" -> Annotation: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#annotation";
    "ornaments" -> Ornaments: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#ornaments";
    "stylistic" -> Stylistic: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#stylistic";
    "styleset" -> Styleset: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#styleset";
    "character-variant" -> CharacterVariant: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-feature-values#character-variant";
    "font-palette-values" -> FontPaletteValues: "https://developer.mozilla.org/en-US/docs/Web/CSS/@font-palette-values";
    "property" -> Property: "https://developer.mozilla.org/en-US/docs/Web/CSS/@property"
);

pub(crate) fn parse_at_rule<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    let mut ident_tokens = tokenizer.iter.peeking_take_while(|t| {
        !matches!(
            t.kind,
            TokenKind::Whitespace
                | TokenKind::EscapedNewline
                | TokenKind::Newline
                | TokenKind::OpenParenthesis
                | TokenKind::OpenBrace
                | TokenKind::Semicolon(_)
        )
    });

    let start_token = match ident_tokens.next() {
        Some(token) => token,
        None => {
            // TODO: set log for invalid at-rule
            return false;
        }
    };
    let end_token = match ident_tokens.last() {
        Some(token) => token,
        None => start_token,
    };
    let ident = &start_token.input[start_token.offset.start..end_token.offset.end];
    let at_rule_ident = match AtRuleId::try_from(ident) {
        Ok(id) => id,
        Err(_) => {
            // TODO: set log for unsupported at-rule
            return false;
        }
    };
    attrb_tokens.push(AttributeToken {
        kind: AttributeTokenKind::AtRuleIdent(at_rule_ident),
        start: start_token.start,
        end: end_token.end,
    });

    todo!()
}

/// At-rule parser for `media` and `container`.
fn parse_conditional<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}

fn parse_conditional_prelude<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}

fn parse_token_parts<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
    scope: usize,
    quote: Option<TokenKind>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    let mut tokens = tokenizer.iter.peeking_take_while(|t| match quote {
        Some(quote_token) => {
            t.kind != quote_token
                && !matches!(t.kind, TokenKind::Newline | TokenKind::EscapedNewline)
        }
        None => !matches!(
            t.kind,
            TokenKind::OpenBrace
                | TokenKind::SingleQuote
                | TokenKind::DoubleQuote
                | TokenKind::Newline
                | TokenKind::EscapedNewline
        ),
    });
    let start_token = match tokens.next() {
        Some(token) => token,
        None => {
            // TODO: set log for invalid at-rule
            return false;
        }
    };
    let end_token = match tokens.last() {
        Some(token) => token,
        None => start_token,
    };
    let part = &start_token.input[start_token.offset.start..end_token.offset.end];

    todo!()
}

fn parse_layer<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}

fn parse_scope<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}

fn parse_keyframes<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}

fn parse_page<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}

fn parse_supports<'slice, 'input, P, T, C>(
    tokenizer: &mut AttributeTokenizer<'slice, 'input, P, T, C>,
    attrb_tokens: &mut Vec<AttributeToken>,
) -> bool
where
    P: Parser<'slice, 'input, T, C>,
    T: Element,
    C: Default,
{
    todo!()
}
