use Fixed;
use tape::Value;

const MAGIC_NUMBER: u32 = 0x5F0F3CF5;

table! {
    #[doc = "A font header."]
    #[derive(Copy)]
    pub FontHeader {
        version (Fixed) |tape, this| {
            let value = try!(Value::read(tape));
            if value != Fixed(0x00010000) {
                raise!("the version of the font header is not supported");
            }
            Ok(value)
        },

        revision            (Fixed), // fontRevision
        checksum_adjustment (u32  ), // checkSumAdjustment

        magic_number (u32) |tape, this| { // MagicNumber
            let value = try!(Value::read(tape));
            if value != MAGIC_NUMBER {
                raise!("the font header is corrupted");
            }
            Ok(value)
        },

        flags                 (u16),
        units_per_em          (u16), // unitsPerEm
        created               (i64),
        modified              (i64),
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
