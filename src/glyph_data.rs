use {Result, Tape, Value, Walue};

#[derive(Clone, Debug)]
pub struct GlyphData(pub Vec<Glyph>);

table! {
    #[doc = "A glyph."]
    pub Glyph {
        contour_count (i16), // numberOfContours
        min_x         (i16), // xMin
        min_y         (i16), // yMin
        max_x         (i16), // xMax
        max_y         (i16), // yMax

        description (GlyphDescription) |tape, this| {
            Walue::read(tape, this.contour_count)
        },
    }
}

/// A glyph description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphDescription {
    Simple(SimpleGlyphDescription),
    Composit(CompositGlyphDescription),
}

table! {
    @define
    #[doc = "A simple-glyph description."]
    pub SimpleGlyphDescription {
        end_points         (Vec<u16>        ), // endPtsOfContours
        instruction_length (u16             ), // instructionLength
        instructions       (Vec<u8>         ), // instructions
        flags              (Vec<u8>         ), // flags
        x_coordinates      (GlyphCoordinates), // xCoordinates
        y_coordinates      (GlyphCoordinates), // yCoordinates
    }
}

/// Glyph coordinates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlyphCoordinates {
    UInt8(Vec<u8>),
    Int16(Vec<i16>),
}

table! {
    #[doc = "A composit-glyph description."]
    pub CompositGlyphDescription {
        flags       (u16), // flags
        glyph_index (u16), // glyphIndex

        argument1 (GlyphArgument) |tape, this| { // argument1
            Walue::read(tape, this.flags)
        },

        argument2 (GlyphArgument) |tape, this| { // argument2
            Walue::read(tape, this.flags)
        },
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlyphArgument {
    Int8(i8),
    Int16(i16),
    UInt8(u8),
    UInt16(u16),
}

impl Walue<usize> for GlyphData {
    fn read<T: Tape>(band: &mut T, count: usize) -> Result<Self> {
        let mut glyphs = Vec::with_capacity(count);
        for i in 0..count {
            glyphs[i] = try!(Value::read(band));
        }
        Ok(GlyphData(glyphs))
    }
}

impl Default for GlyphDescription {
    #[inline]
    fn default() -> Self {
        GlyphDescription::Simple(Default::default())
    }
}

impl Walue<i16> for GlyphDescription {
    fn read<T: Tape>(band: &mut T, contour_count: i16) -> Result<Self> {
        if contour_count >= 0 {
            Ok(GlyphDescription::Simple(try!(Walue::read(band, contour_count as usize))))
        } else {
            Ok(GlyphDescription::Composit(try!(Value::read(band))))
        }
    }
}

impl Walue<usize> for SimpleGlyphDescription {
    fn read<T: Tape>(_: &mut T, _: usize) -> Result<Self> {
        unreachable!()
    }
}

impl Default for GlyphCoordinates {
    #[inline]
    fn default() -> Self {
        GlyphCoordinates::UInt8(Default::default())
    }
}

impl Walue<u16> for GlyphCoordinates {
    fn read<T: Tape>(_: &mut T, _: u16) -> Result<Self> {
        unreachable!()
    }
}

impl Default for GlyphArgument {
    #[inline]
    fn default() -> Self {
        GlyphArgument::Int8(Default::default())
    }
}

impl Walue<u16> for GlyphArgument {
    fn read<T: Tape>(_: &mut T, _: u16) -> Result<Self> {
        unreachable!()
    }
}
