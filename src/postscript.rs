use {Number, Result, Tape, Value};

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
        version             (Number), // version
        italic_angle        (Number), // italicAngle
        underline_position  (i16   ), // underlinePosition
        underline_thickness (i16   ), // underlineThickness
        is_fixed_pitch      (u32   ), // isFixedPitch
        min_memory_type42   (u32   ), // minMemType42
        max_memory_type42   (u32   ), // maxMemType42
        min_memory_type1    (u32   ), // minMemType1
        max_memory_type1    (u32   ), // maxMemType1
    }
}

/// PostScript information of version 3.0.
pub type PostScript30 = PostScript10;

impl Value for PostScript {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<Number>()) {
            Number(0x00010000) => PostScript::Version10(try!(Value::read(tape))),
            Number(0x00030000) => PostScript::Version30(try!(Value::read(tape))),
            _ => raise!("the format of the PostScript information is not supported"),
        })
    }
}
