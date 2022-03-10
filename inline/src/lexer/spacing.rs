use std::ops::{Add, AddAssign};

/// Spacing around the given Token.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Spacing {
    /// Whitespace before the token.
    Pre,

    /// Whitespace after the token.
    Post,

    /// Whitespace before and after the token.
    Both,

    /// Whitespace neither before nor after the token.
    Neither,
}

impl Add for Spacing {
    type Output = Spacing;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Spacing::Pre, Spacing::Post) => Spacing::Both,
            (Spacing::Post, Spacing::Pre) => Spacing::Both,
            (_, Spacing::Both) => rhs,
            (_, Spacing::Neither) => self,
            (Spacing::Both, _) => self,
            (Spacing::Neither, _) => rhs,
            // in other cases is true that 'self == rhs'
            _ => self,
        }
    }
}

impl AddAssign for Spacing {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}
