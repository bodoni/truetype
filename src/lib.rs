//! Parser for TrueType fonts.

#[macro_use]
mod macros;

mod char_mapping;
mod fixed;
mod font_header;
mod horizontal_header;
mod horizontal_metrics;
mod maximum_profile;
mod naming_table;
mod offset_table;
mod postscript;
mod tag;
mod tape;
mod windows_metrics;

pub use char_mapping::{
    CharMapping,
    CharMappingEncoding,
    CharMappingEncoding4,
    CharMappingEncoding6,
    CharMappingHeader,
    CharMappingRecord,
};
pub use fixed::Fixed;
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
    LanguageTagRecord,
    NameRecord,
    NamingTable,
    NamingTable0,
    NamingTable1,
};
pub use offset_table::{
    OffsetTable,
    OffsetTableHeader,
    OffsetTableRecord,
};
pub use postscript::{
    PostScript,
    PostScript10,
    PostScript30,
};
pub use tag::Tag;
pub use tape::{Tape, Value};
pub use windows_metrics::{
    WindowsMetrics,
    WindowsMetrics3,
    WindowsMetrics5,
};

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;
