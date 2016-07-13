//! Parser for TrueType fonts.

#[macro_use]
mod macros;

mod char_mapping;
mod font_header;
mod horizontal_header;
mod horizontal_metrics;
mod maximum_profile;
mod naming_table;
mod number;
mod offset_table;
mod postscript;
mod tag;
mod tape;
mod windows_metrics;

pub mod glyph;

pub use char_mapping::{
    CharMapping,
    CharMappingHeader,
    CharMappingRecord,
    EncodingRecord,
    EncodingRecord4,
    EncodingRecord6,
};
pub use font_header::FontHeader;
pub use horizontal_header::HorizontalHeader;
pub use horizontal_metrics::{
    HorizontalMetrics,
    HorizontalMetricRecord,
};
pub use maximum_profile::{
    MaximumProfile,
    MaximumProfile05,
    MaximumProfile10,
};
pub use naming_table::{
    LanguageRecord,
    NamingRecord,
    NamingTable,
    NamingTable0,
    NamingTable1,
};
pub use number::Number;
pub use offset_table::{
    OffsetHeader,
    OffsetRecord,
    OffsetTable,
};
pub use postscript::{
    PostScript,
    PostScript10,
    PostScript30,
};
pub use tag::Tag;
pub use tape::{Tape, Value, Walue};
pub use windows_metrics::{
    WindowsMetrics,
    WindowsMetrics3,
    WindowsMetrics5,
};

/// An error.
pub type Error = std::io::Error;

/// Glyph data.
pub type GlyphData = Vec<glyph::Glyph>;

/// A result.
pub type Result<T> = std::io::Result<T>;
