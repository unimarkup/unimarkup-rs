/// ASCII Emojis that can be replaced with their Unicode versions in a Unimarkup text.
pub const EMOJIS: [(&str, &str); 18] = [
    (":)", "1F642"),
    (";)", "1F609"),
    (":D", "1F603"),
    ("^^", "1F604"),
    ("=)", "1F60A"),
    (":(", "1F641"),
    (";(", "1F622"),
    (":P", "1F61B"),
    (";P", "1F61C"),
    ("O:)", "1F607"),
    (":O", "1F628"),
    (">:(", "1F92C"),
    (":/", "1F615"),
    ("3:)", "1F608"),
    ("--", "1F611"),
    ("<3", "2764"),
    ("(Y)", "1F44D"),
    ("(N)", "1F44E"),
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

/// Substitution found in a Unimarkup document. Using the implementation of the [`From<&str>`] trait
/// it is possible to generate a `Substitute` for some given input.
///
/// If there’s no defined substitution for given input in Unimarkup specification, a Substitute with
/// original input as its content is generated.
pub struct Substitute {
    content: String,
    original_len: usize,
}

impl<T> From<T> for Substitute
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        let mut content = input.into();
        let original_len = content.len();

        Self::substitute(&mut content);

        Self {
            content,
            original_len,
        }
    }
}

impl Substitute {
    /// Substitutes all occurrences of [`EMOJIS`] and [`ALIASES`] with their Unicode values in place.
    ///
    /// [`EMOJIS`]: self::EMOJIS
    /// [`ALIASES`]: self::ALIASES
    pub fn substitute(content: &mut String) {
        for (key, value) in EMOJIS
            .into_iter()
            .chain(ALIASES.into_iter())
            .chain(ARROWS.into_iter())
        {
            *content = content.replace(key, value);
        }
    }

    /// Returns the content of this Substitute as &str.
    pub fn as_str(&self) -> &str {
        &self.content
    }

    /// Returns length of the content of this Substitute.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Returns the length of the content of this Substitute before substitutions have taken place.
    pub fn original_len(&self) -> usize {
        self.original_len
    }
}
