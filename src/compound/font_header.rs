use primitive::Fixed;
use tape::Value;

const MAGIC_NUMBER: u32 = 0x5F0F3CF5;

macro_rules! read_magic_number(
    ($tape:ident) => ({
        let number = try!(Value::read($tape));
        if number != MAGIC_NUMBER {
            raise!("the font header is corrupted");
        }
        Ok(number)
    });
);

table! {
    #[doc = "A font header."]
    #[derive(Copy)]
    pub FontHeader {
        version            (Fixed),
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
