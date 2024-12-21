//! Builder and parser of TrueType fonts.

#[macro_use(choices, dereference, flags, jump_take, raise, table)]
extern crate typeface;

pub mod tables;

mod tag;

pub use typeface::{q16, q32, tape, value, walue, Error, Result};

pub use tag::Tag;

/// A glyph identifier.
pub type GlyphID = u16;

/// Check if a tag is recognized.
#[inline]
pub fn accept(tag: &Tag) -> bool {
    tables::offsets::Header::accept(tag)
}
