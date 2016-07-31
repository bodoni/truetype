//! The [PostScript information][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/post.htm

use {Result, Tape, Value, Walue, q32};

/// PostScript information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostScript {
    /// Version 1.0.
    Version10(Version10),
    /// Version 2.0.
    Version20(Version20),
    /// Version 3.0.
    Version30(Version30),
}

table! {
    #[doc = "PostScript information of version 1.0."]
    #[derive(Copy)]
    pub Version10 {
        version             (q32), // version
        italic_angle        (q32), // italicAngle
        underline_position  (i16), // underlinePosition
        underline_thickness (i16), // underlineThickness
        is_fixed_pitch      (u32), // isFixedPitch
        min_memory_type42   (u32), // minMemType42
        max_memory_type42   (u32), // maxMemType42
        min_memory_type1    (u32), // minMemType1
        max_memory_type1    (u32), // maxMemType1
    }
}

table! {
    #[doc = "PostScript information of version 2.0."]
    pub Version20 {
        version             (q32), // version
        italic_angle        (q32), // italicAngle
        underline_position  (i16), // underlinePosition
        underline_thickness (i16), // underlineThickness
        is_fixed_pitch      (u32), // isFixedPitch
        min_memory_type42   (u32), // minMemType42
        max_memory_type42   (u32), // maxMemType42
        min_memory_type1    (u32), // minMemType1
        max_memory_type1    (u32), // maxMemType1

        glyph_count (u16), // numberOfGlyphs

        glyph_name_indices (Vec<u16>) |tape, this| { // glyphNameIndex
            Walue::read(tape, this.glyph_count as usize)
        },

        glyph_names (Vec<String>) |tape, this| { // names
            read_pascal_strings(tape, &this.glyph_name_indices)
        },
    }
}

/// PostScript information of version 3.0.
pub type Version30 = Version10;

impl Value for PostScript {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<q32>()) {
            q32(0x00010000) => PostScript::Version10(try!(tape.take())),
            q32(0x00020000) => PostScript::Version20(try!(tape.take())),
            q32(0x00030000) => PostScript::Version30(try!(tape.take())),
            _ => raise!("the format of the PostScript information is not supported"),
        })
    }
}

fn read_pascal_strings<T: Tape>(tape: &mut T, indices: &[u16]) -> Result<Vec<String>> {
    let count = indices.iter().fold(0, |n, &i| if 258 <= i && i <= 32767 { n + 1 } else { n });
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        match String::from_utf8(read_bytes!(tape, try!(tape.take::<u8>()) as usize)) {
            Ok(name) => names.push(name),
            _ => names.push("<malformed>".into()),
        }
    }
    Ok(names)
}
