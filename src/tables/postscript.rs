//! The [PostScript table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/post

use crate::{q32, Result};

/// A PostScript table.
#[derive(Clone, Debug)]
pub enum PostScript {
    /// Version 1.
    Version1(PostScript1),
    /// Version 2.
    Version2(PostScript2),
    /// Version 3.
    Version3(PostScript3),
}

table! {
    /// A PostScript table of version 1.
    #[derive(Copy)]
    pub PostScript1 {
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
    /// A PostScript table of version 2.
    pub PostScript2 {
        version             (q32), // version
        italic_angle        (q32), // italicAngle
        underline_position  (i16), // underlinePosition
        underline_thickness (i16), // underlineThickness
        is_fixed_pitch      (u32), // isFixedPitch
        min_memory_type42   (u32), // minMemType42
        max_memory_type42   (u32), // maxMemType42
        min_memory_type1    (u32), // minMemType1
        max_memory_type1    (u32), // maxMemType1
        glyph_count         (u16), // numberOfGlyphs

        glyph_name_indices (Vec<u16>) |this, tape| { // glyphNameIndex
            tape.take_given(this.glyph_count as usize)
        },

        glyph_names (Vec<String>) |this, tape| { // names
            read_pascal_strings(tape, &this.glyph_name_indices)
        },
    }
}

/// A PostScript table of version 3.
pub type PostScript3 = PostScript1;

impl crate::value::Read for PostScript {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<q32>()? {
            q32(0x00010000) => PostScript::Version1(tape.take()?),
            q32(0x00020000) => PostScript::Version2(tape.take()?),
            q32(0x00030000) => PostScript::Version3(tape.take()?),
            _ => raise!("found an unknown version of the PostScript table"),
        })
    }
}

fn read_pascal_strings<T: crate::tape::Read>(tape: &mut T, indices: &[u16]) -> Result<Vec<String>> {
    let count = indices.iter().fold(
        0,
        |n, &i| if (258..=32767).contains(&i) { n + 1 } else { n },
    );
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        let size = tape.take::<u8>()? as usize;
        match String::from_utf8(tape.take_bytes(size)?) {
            Ok(name) => names.push(name),
            _ => names.push("<malformed>".into()),
        }
    }
    Ok(names)
}
