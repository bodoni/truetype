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
        version            (Fixed) |tape, this| { read_version!(tape) },
        fontRevision       (Fixed),
        checkSumAdjustment (u32  ),
        magicNumber        (u32  ) |tape, this| { read_magic_number!(tape) },
        flags              (u16  ),
        unitsPerEm         (u16  ),
        created            (i64  ),
        modified           (i64  ),
        xMin               (i16  ),
        yMin               (i16  ),
        xMax               (i16  ),
        yMax               (i16  ),
        macStyle           (u16  ),
        lowestRecPPEM      (u16  ),
        fontDirectionHint  (i16  ),
        indexToLocFormat   (i16  ),
        glyphDataFormat    (i16  ),
    }
}
