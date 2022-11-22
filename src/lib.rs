//! Parser for TrueType fonts.

#[macro_use]
mod macros;

mod number;
mod tag;
mod tape;

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
pub use tape::{Tape, Value, Walue};
pub use windows_metrics::WindowsMetrics;

/// An error.
pub type Error = std::io::Error;

/// An error caused by another error.
#[derive(Debug)]
pub struct ErrorWithSource {
    pub description: String,
    pub source: Error,
}

/// A glyph identifier.
pub type GlyphID = u16;

/// A result.
pub type Result<T> = std::io::Result<T>;

impl std::fmt::Display for ErrorWithSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}, due to {}", self.description, self.source)
    }
}

impl std::error::Error for ErrorWithSource {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
