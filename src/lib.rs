//! Parser for TrueType fonts.

use std::mem;

#[macro_use]
mod macros;

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

/// A fixed-point number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

/// A font-table tag.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Tag(pub u32);

impl From<Fixed> for f32 {
    #[inline]
    fn from(fixed: Fixed) -> Self {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (fixed.0 as f32)
    }
}

impl From<Tag> for [u8; 4] {
    #[inline]
    fn from(tag: Tag) -> Self {
        unsafe { mem::transmute(u32::from_be(tag.0)) }
    }
}

impl From<[u8; 4]> for Tag {
    #[inline]
    fn from(bytes: [u8; 4]) -> Self {
        Tag(u32::from_be(unsafe { mem::transmute(bytes) }))
    }
}

impl<'l> From<&'l [u8; 4]> for Tag {
    #[inline(always)]
    fn from(bytes: &'l [u8; 4]) -> Self {
        (*bytes).into()
    }
}

impl From<Fixed> for Tag {
    #[inline(always)]
    fn from(fixed: Fixed) -> Self {
        Tag(fixed.0)
    }
}

macro_rules! implement {
    ($name:ident, 1) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok(read!(tape, 1))
        }
    });
    ($name:ident, $size:expr) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok($name::from_be(read!(tape, $size)))
        }
    });
}

implement!(i16, 2);
implement!(u16, 2);
implement!(u32, 4);
implement!(i64, 8);

impl Value for Fixed {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Fixed(try!(Value::read(tape))))
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn from() {
        assert_eq!(Tag::from(b"true"), Tag(0x74727565));
    }
}
