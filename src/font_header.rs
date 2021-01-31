//! The [font header][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/head.htm

use crate::q32;

table! {
    #[doc = "A font header."]
    #[derive(Copy)]
    pub FontHeader {
        major_version        (u16  ) = { 1 }, // majorVersion
        minor_version        (u16  ) = { 0 }, // minorVersion
        revision             (q32  ), // fontRevision
        checksum_adjustment  (u32  ), // checkSumAdjustment
        magic_number         (u32  ) = { 0x5F0F3CF5 }, // MagicNumber
        flags                (Flags), // flags
        units_per_em         (u16  ), // unitsPerEm
        created              (i64  ), // created
        modified             (i64  ), // modified
        min_x                (i16  ), // xMin
        min_y                (i16  ), // yMin
        max_x                (i16  ), // xMax
        max_y                (i16  ), // yMax
        mac_style            (u16  ), // macStyle
        lowest_ppem          (u16  ), // lowestRecPPEM
        direction_hint       (i16  ), // fontDirectionHint
        glyph_mapping_format (i16  ), // indexToLocFormat
        glyph_data_format    (i16  ), // glyphDataFormat
    }
}

flags! {
    #[doc = "Font-header flags."]
    pub Flags(u16) {
        0b0000_0000_0000_0001 => is_baseline_at_0,
        0b0000_0000_0000_0010 => is_left_side_bearing_at_0,
        0b1000_0000_0000_0000 => is_invalid,
    }
}
