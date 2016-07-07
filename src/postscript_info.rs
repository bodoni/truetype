use {Fixed, Result};
use tape::{Tape, Value};

/// PostScript information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostScriptInfo {
    /// Version 1.0.
    Version10(PostScriptInfo10),
    /// Version 3.0.
    Version30(PostScriptInfo30),
}

table! {
    #[doc = "PostScript information of version 1.0."]
    #[derive(Copy)]
    pub PostScriptInfo10 {
        version             (Fixed),
        italic_angle        (Fixed), // italicAngle
        underline_position  (i16  ), // underlinePosition
        underline_thickness (i16  ), // underlineThickness
        is_fixed_pitch      (u32  ), // isFixedPitch
        min_memory_type42   (u32  ), // minMemType42
        max_memory_type42   (u32  ), // maxMemType42
        min_memory_type1    (u32  ), // minMemType1
        max_memory_type1    (u32  ), // maxMemType1
    }
}

/// PostScript information of version 3.0.
pub type PostScriptInfo30 = PostScriptInfo10;

impl Value for PostScriptInfo {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<Fixed>()) {
            Fixed(0x00010000) => PostScriptInfo::Version10(try!(Value::read(tape))),
            Fixed(0x00030000) => PostScriptInfo::Version30(try!(Value::read(tape))),
            _ => raise!("the format of the PostScript information is not supported"),
        })
    }
}
