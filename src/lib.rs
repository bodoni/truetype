//! Parser for TrueType fonts.

#[macro_use]
mod macros;

mod number;
mod tag;
mod tape;

pub mod char_mapping;
pub mod font_header;
pub mod glyph_data;
pub mod horizontal_header;
pub mod horizontal_metrics;
pub mod maximum_profile;
pub mod naming_table;
pub mod offset_table;
pub mod postscript;
pub mod windows_metrics;

pub use char_mapping::CharMapping;
pub use font_header::FontHeader;
pub use glyph_data::GlyphData;
pub use horizontal_header::HorizontalHeader;
pub use horizontal_metrics::HorizontalMetrics;
pub use maximum_profile::MaximumProfile;
pub use naming_table::NamingTable;
pub use number::Number;
pub use offset_table::OffsetTable;
pub use postscript::PostScript;
pub use tag::Tag;
pub use tape::{Tape, Value, Walue};
pub use windows_metrics::WindowsMetrics;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;
