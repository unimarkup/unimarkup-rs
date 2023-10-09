//! Substitutor and constants that can be substituted in Unimarkup content.

use std::collections::{HashMap, HashSet};

use logid::evident::once_cell::sync::Lazy;
use unimarkup_commons::scanner::{self, span::Span};

/// ASCII Emojis that can be replaced with their Unicode versions in a Unimarkup text.
pub const EMOJIS: [(&str, &str); 18] = [
    (":)", "\u{1F642}"),
    (";)", "\u{1F609}"),
    (":D", "\u{1F603}"),
    ("^^", "\u{1F604}"),
    ("=)", "\u{1F60A}"),
    (":(", "\u{1F641}"),
    (";(", "\u{1F622}"),
    (":P", "\u{1F61B}"),
    (";P", "\u{1F61C}"),
    ("O:)", "\u{1F607}"),
    (":O", "\u{1F628}"),
    (">:(", "\u{1F92C}"),
    (":/", "\u{1F615}"),
    ("3:)", "\u{1F608}"),
    ("--", "\u{1F611}"),
    ("<3", "\u{2764}"),
    ("(Y)", "\u{1F44D}"),
    ("(N)", "\u{1F44E}"),
];

/// ASCII Arrows that can be replaced with their Unicode versions in a Unimarkup text.
pub const ARROWS: [(&str, &str); 18] = [
    ("-->", "\u{1F816}"),
    ("|-->", "\u{21A6}"),
    ("---->", "\u{27F6}"),
    ("|---->", "\u{27FC}"),
    ("==>", "\u{21D2}"),
    ("|==>", "\u{2907}"),
    ("====>", "\u{27F9}"),
    ("|====>", "\u{27FE}"),
    ("<--", "\u{1F814}"),
    ("<--|", "\u{21A4}"),
    ("<----", "\u{27F5}"),
    ("<----|", "\u{27FB}"),
    ("<==", "\u{21D0}"),
    ("<==|", "\u{2906}"),
    ("<====", "\u{27F8}"),
    ("<====|", "\u{27F8}"),
    ("<-->", "\u{27F7}"),
    ("<==>", "\u{21D4}"),
];

/// Aliases for the [`EMOJIS`] and [`ARROWS`] that can be replaced in a Unimarkup text.
///
/// [`EMOJIS`]: self::EMOJIS
/// [`ARROWS`]: self::ARROWS
pub const ALIASES: [(&str, &str); 20] = [
    ("::slightly_smiling_face::", "\u{1F642}"),
    ("::wink::", "\u{1F609}"),
    ("::smiley::", "\u{1F603}"),
    ("::smile::", "\u{1F604}"),
    ("::blush::", "\u{1F60A}"),
    ("::slightly_frowning_face::", "\u{1F641}"),
    ("::cry::", "\u{1F622}"),
    ("::stuck_out_tongue::", "\u{1F61B}"),
    ("::stuck_out_tongue_winking_eye::", "\u{1F61C}"),
    ("::innocent::", "\u{1F607}"),
    ("::fearful::", "\u{1F628}"),
    ("::cursing_face::", "\u{1F92C}"),
    ("::confused::", "\u{1F615}"),
    ("::smiling_imp::", "\u{1F608}"),
    ("::expressionless::", "\u{1F611}"),
    ("::heart::", "\u{2764}"),
    ("::+1::", "\u{1F44D}"),
    ("::thumbsup::", "\u{1F44D}"),
    ("::-1::", "\u{1F44E}"),
    ("::thumbsdown::", "\u{1F44E}"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Substitutor<'a> {
    direct: HashMap<&'a str, &'a str>,
    aliased: HashMap<&'a str, &'a str>,
    max_len: usize,
    first_grapheme: HashSet<&'a str>,
}

static GLOBAL: Lazy<Substitutor> = Lazy::new(Substitutor::create_global);

impl<'sub> Substitutor<'sub> {
    fn create_global() -> Substitutor<'static> {
        let direct: HashMap<_, _> = EMOJIS.into_iter().chain(ARROWS).collect();
        let aliased = ALIASES.into_iter().collect();
        let max_len = direct.keys().map(|key| key.len()).max().unwrap_or(0);
        let first_grapheme = direct.keys().map(|key| &key[0..1]).collect();

        Substitutor {
            direct,
            aliased,
            max_len,
            first_grapheme,
        }
    }

    pub fn global() -> &'static Substitutor<'static> {
        &GLOBAL
    }

    pub(crate) fn get_subst(&self, slice: &'sub str, span: Span) -> Option<Substitute<'sub>> {
        let content = *self.direct.get(slice)?;

        Some(Substitute { content, span })
    }

    pub(crate) fn get_subst_iter<I>(&self, iter: I) -> Option<Substitute<'sub>>
    where
        I: Iterator<Item = &'sub scanner::Symbol<'sub>> + Clone,
    {
        let slice = scanner::Symbol::flatten_iter(iter.clone())?;
        let mut tmp_iter = iter;
        let first = tmp_iter.next()?;
        let start = first.start;
        let end = tmp_iter.last().map_or(first.end, |sym| sym.end);

        let span = Span::from((start, end));

        self.get_subst(slice, span)
    }

    pub(crate) fn is_start_of_substitute(&self, symbol: &scanner::Symbol) -> bool {
        self.first_grapheme.contains(symbol.as_str())
    }

    pub(crate) fn max_len(&self) -> usize {
        self.max_len
    }
}

/// Substitution found in a Unimarkup document. Using the implementation of the [`From<&str>`] trait
/// it is possible to generate a `Substitute` for some given input.
///
/// If there’s no defined substitution for given input in Unimarkup specification, a Substitute with
/// original input as its content is generated.
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Substitute<'sub> {
    content: &'sub str,
    span: Span,
}

impl<'sub> Substitute<'sub> {
    /// Returns the content of this Substitute as &str.
    pub fn as_str(&self) -> &'sub str {
        self.content
    }

    /// Returns the length of the content of this Substitute before substitutions have taken place.
    pub fn span(&self) -> Span {
        self.span
    }
}
