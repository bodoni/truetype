//! Parser for TrueType fonts.

#[macro_use(choices, dereference, flags, jump_take, raise, table)]
extern crate typeface;

pub mod tables;

mod number;
mod tag;

pub use typeface::{Error, Result, Tape, Value, Walue};

pub use number::{q16, q32};
pub use tag::Tag;

/// A glyph identifier.
pub type GlyphID = u16;

/// Check if a tag is recognized.
#[inline]
pub fn accept(tag: &Tag) -> bool {
    tables::offsets::Header::accept(tag)
}
