mod bold;
mod bolditalic;
mod italic;

pub use bold::*;
pub use bolditalic::*;
pub use italic::*;
use unimarkup_commons::scanner::SymbolKind;

pub const BOLD_ITALIC_KEYWORD_LIMIT: &[SymbolKind] = &[
    SymbolKind::Star,
    SymbolKind::Star,
    SymbolKind::Star,
    SymbolKind::Star,
];
