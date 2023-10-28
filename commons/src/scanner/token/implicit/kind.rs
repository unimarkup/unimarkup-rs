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
    DirectUri,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowSubsitution {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmojiSubstitution {}
