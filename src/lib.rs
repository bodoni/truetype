//! Parser for TrueType fonts.

#[macro_use(choices, dereference, flags, raise, table)]
extern crate typeface;

pub mod char_mapping;
pub mod font_header;
pub mod glyph_data;
pub mod glyph_mapping;
pub mod horizontal_header;
pub mod horizontal_metrics;
pub mod maximum_profile;
pub mod naming_table;
pub mod offset_table;
pub mod postscript;
pub mod windows_metrics;

mod number;
mod tag;

pub use typeface::{Error, Result, Tape, Value, Walue};

pub use char_mapping::CharMapping;
pub use font_header::FontHeader;
pub use glyph_data::GlyphData;
pub use glyph_mapping::GlyphMapping;
pub use horizontal_header::HorizontalHeader;
pub use horizontal_metrics::HorizontalMetrics;
pub use maximum_profile::MaximumProfile;
pub use naming_table::NamingTable;
pub use number::{q16, q32};
pub use offset_table::OffsetTable;
pub use postscript::PostScript;
pub use tag::Tag;
pub use windows_metrics::WindowsMetrics;

/// A glyph identifier.
pub type GlyphID = u16;
