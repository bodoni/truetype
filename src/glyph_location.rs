//! The index-to-location table.

use {Result, Tape, Walue};

/// An index-to-location table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphLocation {
    /// Short offsets divided by two.
    Short(Vec<u16>),
    /// Long offsets.
    Long(Vec<u32>),
}

impl Walue<(i32, usize)> for GlyphLocation {
    fn read<T: Tape>(tape: &mut T, (glyph_location_format, glyph_count): (i32, usize))
                     -> Result<Self> {

        match glyph_location_format {
            0 => Ok(GlyphLocation::Short(read_walue!(tape, glyph_count + 1))),
            1 => Ok(GlyphLocation::Long(read_walue!(tape, glyph_count + 1))),
            _ => raise!("the index-to-location format is unknown"),
        }
    }
}
