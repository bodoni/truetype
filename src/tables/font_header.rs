//! The [font header][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/head

use crate::number::q32;
use crate::Result;

table! {
    @write
    /// A font header.
    #[derive(Copy)]
    pub FontHeader {
        major_version        (u16           ) = { 1 }, // majorVersion
        minor_version        (u16           ) = { 0 }, // minorVersion
        revision             (q32           ), // fontRevision
        checksum_adjustment  (u32           ), // checkSumAdjustment
        magic_number         (u32           ) = { 0x5F0F3CF5 }, // MagicNumber
        flags                (Flags         ), // flags
        units_per_em         (u16           ), // unitsPerEm
        created              (i64           ), // created
        modified             (i64           ), // modified
        min_x                (i16           ), // xMin
        min_y                (i16           ), // yMin
        max_x                (i16           ), // xMax
        max_y                (i16           ), // yMax
        macintosh_flags      (MacintoshFlags), // macStyle
        lowest_ppem          (u16           ), // lowestRecPPEM
        direction_hint       (i16           ), // fontDirectionHint
        glyph_mapping_format (i16           ), // indexToLocFormat
        glyph_data_format    (i16           ), // glyphDataFormat
    }
}

flags! {
    /// Font-header flags.
    pub Flags(u16) {
        0b0000_0000_0000_0001 => is_baseline_at_0,
        0b0000_0000_0000_0010 => is_left_side_bearing_at_0,
        0b1000_0000_0000_0000 => is_invalid,
    }
}

flags! {
    /// Macintosh style flags.
    pub MacintoshFlags(u16) {
        0b0000_0000_0000_0001 => is_bold,
        0b0000_0000_0000_0010 => is_italic,
        0b0000_0000_0000_0100 => is_underline,
        0b0000_0000_0000_1000 => is_outline,
        0b0000_0000_0001_0000 => is_shadow,
        0b0000_0000_0010_0000 => is_condensed,
        0b0000_0000_0100_0000 => is_extended,
        0b1111_1111_1000_0000 => is_invalid,
    }
}

impl FontHeader {
    /// The magic number for computing the checksum adjustment.
    pub const CHECKSUM_ADJUSTMENT: u32 = 0xB1B0AFBA;

    /// Compute the checksum.
    pub fn checksum<T: crate::tape::Read>(tape: &mut T) -> Result<u32> {
        let mut data = vec![];
        tape.read_to_end(&mut data)?;
        if data.len() % 4 != 0 {
            raise!("found a malformed table layout");
        }
        let sum = data
            .chunks_exact(4)
            .map(TryInto::try_into)
            .map(std::result::Result::unwrap)
            .map(u32::from_be_bytes)
            .fold(0u32, |sum, value| sum.wrapping_add(value));
        Ok(Self::CHECKSUM_ADJUSTMENT.wrapping_sub(sum))
    }
}
