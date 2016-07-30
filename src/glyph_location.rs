//! The [index-to-location table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/loca.htm

use {FontHeader, MaximumProfile, Result, Tape, Walue};

/// An index-to-location table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphLocation {
    /// Offsets devided by two.
    HalfOffsets(Vec<u16>),
    /// Offsets.
    Offsets(Vec<u32>),
}

impl<'l> Walue<(&'l FontHeader, &'l MaximumProfile)> for GlyphLocation {
    fn read<T: Tape>(tape: &mut T, (header, profile): (&FontHeader, &MaximumProfile))
                     -> Result<Self> {

        let glyph_count = profile.glyph_count();
        match header.glyph_location_format {
            0 => Ok(GlyphLocation::HalfOffsets(read_walue!(tape, glyph_count + 1))),
            1 => Ok(GlyphLocation::Offsets(read_walue!(tape, glyph_count + 1))),
            _ => raise!("the index-to-location format is unknown"),
        }
    }
}
