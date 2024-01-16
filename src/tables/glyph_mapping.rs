//! The [glyph-to-location mapping][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/loca

use crate::tables::font_header::FontHeader;
use crate::tables::maximum_profile::MaximumProfile;
use crate::{Result, Tape, Walue};

/// A glyph-to-location mapping.
#[derive(Clone, Debug)]
pub enum GlyphMapping {
    /// Offsets devided by two.
    HalfOffsets(Vec<u16>),
    /// Offsets.
    Offsets(Vec<u32>),
}

impl<'l> Walue<'l> for GlyphMapping {
    type Parameter = (&'l FontHeader, &'l MaximumProfile);

    fn read<T: Tape>(tape: &mut T, (header, profile): Self::Parameter) -> Result<Self> {
        let glyph_count = profile.glyph_count();
        match header.glyph_mapping_format {
            0 => Ok(GlyphMapping::HalfOffsets(tape.take_given(glyph_count + 1)?)),
            1 => Ok(GlyphMapping::Offsets(tape.take_given(glyph_count + 1)?)),
            _ => raise!("found an unknown format of the glyph-to-location mapping"),
        }
    }
}
