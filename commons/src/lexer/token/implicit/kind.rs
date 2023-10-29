#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitSubstitution {
    Arrow(ArrowSubsitution),
    Emoji(EmojiSubstitution),
    Trademark,
    Copyright,
    Registered,
    HorizontalEllipsis,
    PlusMinus,
    EnDash,
    EmDash,
}

impl ImplicitSubstitution {
    pub fn orig(&self) -> &'static str {
        match self {
            ImplicitSubstitution::Arrow(_) => todo!(),
            ImplicitSubstitution::Emoji(_) => todo!(),
            ImplicitSubstitution::Trademark => "(TM)",
            ImplicitSubstitution::Copyright => "((C))",
            ImplicitSubstitution::Registered => "((R))",
            ImplicitSubstitution::HorizontalEllipsis => "...",
            ImplicitSubstitution::PlusMinus => "(+-)",
            ImplicitSubstitution::EnDash => "--",
            ImplicitSubstitution::EmDash => "---",
        }
    }

    pub fn subst(&self) -> &'static str {
        match self {
            ImplicitSubstitution::Arrow(_) => todo!(),
            ImplicitSubstitution::Emoji(_) => todo!(),
            ImplicitSubstitution::Trademark => "™",
            ImplicitSubstitution::Copyright => "©",
            ImplicitSubstitution::Registered => "®",
            ImplicitSubstitution::HorizontalEllipsis => "…",
            ImplicitSubstitution::PlusMinus => "±",
            ImplicitSubstitution::EnDash => "–",
            ImplicitSubstitution::EmDash => "—",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowSubsitution {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmojiSubstitution {}
