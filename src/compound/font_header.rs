use primitive::Fixed;
use tape::Value;

const MAGIC_NUMBER: u32 = 0x5F0F3CF5;

macro_rules! read_version(
    ($tape:ident) => ({
        let value = try!(Value::read($tape));
        if value != Fixed(0x00010000) {
            raise!("the version of the font header is not supported");
        }
        Ok(value)
    });
);

macro_rules! read_magic_number(
    ($tape:ident) => ({
        let value = try!(Value::read($tape));
        if value != MAGIC_NUMBER {
            raise!("the font header is corrupted");
        }
        Ok(value)
    });
);

table! {
    #[doc = "A font header."]
    #[derive(Copy)]
    pub FontHeader {
        version             (Fixed) |tape, this| { read_version!(tape) },
        font_revision       (Fixed), // fontRevision
        checksum_adjustment (u32  ), // checkSumAdjustment
        magic_number        (u32  ) |tape, this| { read_magic_number!(tape) }, // MagicNumber
        flags               (u16  ),
        units_per_em        (u16  ), // unitsPerEm
        created             (i64  ),
        modified            (i64  ),
        min_x               (i16  ), // xMin
        min_y               (i16  ), // yMin
        max_x               (i16  ), // xMax
        max_y               (i16  ), // yMax
        mac_style           (u16  ), // macStyle
        lowest_ppem         (u16  ), // lowestRecPPEM
        font_direction_hint (i16  ), // fontDirectionHint
        location_format     (i16  ), // indexToLocFormat
        glyph_data_format   (i16  ), // glyphDataFormat
    }
}
