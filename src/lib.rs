//! Parser for TrueType fonts.

#[macro_use]
mod macros;

mod fixed;
mod primitive;
mod tag;
mod tape;

mod char_mapping;
mod font_header;
mod horizontal_header;
mod horizontal_metrics;
mod maximum_profile;
mod naming_table;
mod offset_table;
mod postscript_info;
mod windows_metrics;

pub use fixed::Fixed;
pub use tag::Tag;
pub use tape::{Tape, Value};

pub use char_mapping::{
    CharMapping,
    CharMappingEncoding,
    CharMappingEncoding4,
    CharMappingEncoding6,
    CharMappingHeader,
    CharMappingRecord,
};
pub use font_header::FontHeader;
pub use horizontal_header::HorizontalHeader;
pub use horizontal_metrics::{
    HorizontalMetrics,
    LongHorizontalMetric,
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
pub use postscript_info::{
    PostScriptInfo,
    PostScriptInfo10,
    PostScriptInfo30,
};
pub use windows_metrics::{
    WindowsMetrics,
    WindowsMetrics3,
    WindowsMetrics5,
};

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::io::Result<T>;
