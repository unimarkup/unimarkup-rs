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
            ImplicitSubstitutionKind::Arrow(_) => todo!(),
            ImplicitSubstitutionKind::Emoji(_) => todo!(),
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
            ImplicitSubstitutionKind::Arrow(_) => todo!(),
            ImplicitSubstitutionKind::Emoji(_) => todo!(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmojiSubstitutionKind {}
