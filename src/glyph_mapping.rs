//! The [glyph-to-location mapping][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/loca.htm

use {FontHeader, MaximumProfile, Result, Tape, Walue};

/// A glyph-to-location mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphMapping {
    /// Offsets devided by two.
    HalfOffsets(Vec<u16>),
    /// Offsets.
    Offsets(Vec<u32>),
}

impl<'l> Walue<(&'l FontHeader, &'l MaximumProfile)> for GlyphMapping {
    fn read<T: Tape>(tape: &mut T, (header, profile): (&FontHeader, &MaximumProfile))
                     -> Result<Self> {

        let glyph_count = profile.glyph_count();
        match header.glyph_mapping_format {
            0 => Ok(GlyphMapping::HalfOffsets(try!(tape.take_given(glyph_count + 1)))),
            1 => Ok(GlyphMapping::Offsets(try!(tape.take_given(glyph_count + 1)))),
            _ => raise!("found an unknown format of the glyph-to-location mapping"),
        }
    }
}
