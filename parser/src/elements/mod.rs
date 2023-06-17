//! Available elements for a Unimarkup document.

pub mod atomic;
pub mod blocks;
pub mod enclosed;
pub mod inlines;
pub mod preamble;
pub mod types;

use self::blocks::Block;

/// Type alias for a vector of [`Block`] enum.
pub type Blocks = Vec<Block>;
