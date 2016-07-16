//! The font header.

use {Value, q32};

const MAGIC_NUMBER: u32 = 0x5F0F3CF5;

table! {
    #[doc = "A font header."]
    #[derive(Copy)]
    pub FontHeader {
        version (q32) |tape, this| { // version
            let value = try!(Value::read(tape));
            if value != q32(0x00010000) {
                raise!("the version of the font header is not supported");
            }
            Ok(value)
        },

        revision            (q32), // fontRevision
        checksum_adjustment (u32), // checkSumAdjustment

        magic_number (u32) |tape, this| { // MagicNumber
            let value = try!(Value::read(tape));
            if value != MAGIC_NUMBER {
                raise!("the font header is corrupted");
            }
            Ok(value)
        },

        flags                 (u16), // flags
        units_per_em          (u16), // unitsPerEm
        created               (i64), // created
        modified              (i64), // modified
        min_x                 (i16), // xMin
        min_y                 (i16), // yMin
        max_x                 (i16), // xMax
        max_y                 (i16), // yMax
        mac_style             (u16), // macStyle
        lowest_ppem           (u16), // lowestRecPPEM
        direction_hint        (i16), // fontDirectionHint
        glyph_location_format (i16), // indexToLocFormat
        glyph_data_format     (i16), // glyphDataFormat
    }
}
