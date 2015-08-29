use Result;
use primitive::Fixed;
use tape::{Tape, Value};

/// PostScript information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostScript {
    /// Version 1.0.
    Version10(PostScript10),
    /// Version 3.0.
    Version30(PostScript30),
}

table! {
    #[doc = "PostScript information of version 1.0."]
    #[derive(Copy)]
    pub PostScript10 {
        version            (Fixed),
        italicAngle        (Fixed),
        underlinePosition  (i16  ),
        underlineThickness (i16  ),
        isFixedPitch       (u32  ),
        minMemType42       (u32  ),
        maxMemType42       (u32  ),
        minMemType1        (u32  ),
        maxMemType1        (u32  ),
    }
}

/// PostScript information of version 3.0.
pub type PostScript30 = PostScript10;

impl Value for PostScript {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<Fixed>()) {
            Fixed(0x00010000) => PostScript::Version10(try!(Value::read(tape))),
            Fixed(0x00030000) => PostScript::Version30(try!(Value::read(tape))),
            _ => raise!("the format of the PostScript information is not supported"),
        })
    }
}
