#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitSubstitutionKind {
    Arrow(ArrowSubsitutionKind),
    Emoji(EmojiSubstitutionKind),
    Trademark,
    Copyright,
    Registered,
    HorizontalEllipsis,
    PlusMinus,
    EnDash,
    EmDash,
}

impl ImplicitSubstitutionKind {
    pub fn orig(&self) -> &'static str {
        match self {
            ImplicitSubstitutionKind::Arrow(arrow) => arrow.orig(),
            ImplicitSubstitutionKind::Emoji(emoji) => emoji.orig(),
            ImplicitSubstitutionKind::Trademark => "(TM)",
            ImplicitSubstitutionKind::Copyright => "((C))",
            ImplicitSubstitutionKind::Registered => "((R))",
            ImplicitSubstitutionKind::HorizontalEllipsis => "...",
            ImplicitSubstitutionKind::PlusMinus => "(+-)",
            ImplicitSubstitutionKind::EnDash => "--",
            ImplicitSubstitutionKind::EmDash => "---",
        }
    }

    pub fn subst(&self) -> &'static str {
        match self {
            ImplicitSubstitutionKind::Arrow(arrow) => arrow.subst(),
            ImplicitSubstitutionKind::Emoji(emoji) => emoji.subst(),
            ImplicitSubstitutionKind::Trademark => "™",
            ImplicitSubstitutionKind::Copyright => "©",
            ImplicitSubstitutionKind::Registered => "®",
            ImplicitSubstitutionKind::HorizontalEllipsis => "…",
            ImplicitSubstitutionKind::PlusMinus => "±",
            ImplicitSubstitutionKind::EnDash => "–",
            ImplicitSubstitutionKind::EmDash => "—",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowSubsitutionKind {}

impl ArrowSubsitutionKind {
    pub fn orig(&self) -> &'static str {
        //TODO: implement once arrows are defined
        ""
    }

    pub fn subst(&self) -> &'static str {
        //TODO: implement once arrows are defined
        ""
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmojiSubstitutionKind {}

impl EmojiSubstitutionKind {
    pub fn orig(&self) -> &'static str {
        //TODO: implement once emojis are defined
        ""
    }

    pub fn subst(&self) -> &'static str {
        //TODO: implement once emojis are defined
        ""
    }
}
