//! The glyph data.

use {Result, Tape, Value, Walue};

/// Glyph data.
pub type GlyphData = Vec<Glyph>;

table! {
    #[doc = "A glyph."]
    pub Glyph {
        contour_count (i16), // numberOfContours
        min_x         (i16), // xMin
        min_y         (i16), // yMin
        max_x         (i16), // xMax
        max_y         (i16), // yMax

        description (Description) |tape, this| {
            Walue::read(tape, this.contour_count)
        },
    }
}

/// A description.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Description {
    Simple(Simple),
    Composit(Composit),
}

table! {
    @define
    #[doc = "A simple-glyph description."]
    pub Simple {
        end_points         (Vec<u16>), // endPtsOfContours
        instruction_length (u16     ), // instructionLength
        instructions       (Vec<u8> ), // instructions
        flags              (Vec<u8> ), // flags
        x                  (Vec<i16>), // xCoordinates
        y                  (Vec<i16>), // yCoordinates
    }
}

table! {
    #[doc = "A composit-glyph description."]
    pub Composit {
        flags (u16), // flags
        index (u16), // glyphIndex

        arguments (Arguments) |tape, this| { // argument1, argument2
            Walue::read(tape, this.flags)
        },
    }
}

/// Arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arguments {
    Int8([i8; 2]),
    Int16([i16; 2]),
    UInt8([u8; 2]),
    UInt16([u16; 2]),
}

impl Default for Arguments {
    #[inline]
    fn default() -> Self {
        Arguments::Int8(Default::default())
    }
}

impl Walue<u16> for Arguments {
    fn read<T: Tape>(_: &mut T, _: u16) -> Result<Self> {
        unreachable!()
    }
}

impl Default for Description {
    #[inline]
    fn default() -> Self {
        Description::Simple(Default::default())
    }
}

impl Walue<i16> for Description {
    fn read<T: Tape>(tape: &mut T, contour_count: i16) -> Result<Self> {
        if contour_count >= 0 {
            Ok(Description::Simple(try!(Walue::read(tape, contour_count as usize))))
        } else {
            Ok(Description::Composit(try!(Value::read(tape))))
        }
    }
}

impl Walue<usize> for Simple {
    fn read<T: Tape>(tape: &mut T, contour_count: usize) -> Result<Self> {
        macro_rules! reject(() => (raise!("found a malformed glyph description")));

        let end_points = try!(<Vec<u16>>::read(tape, contour_count));
        for i in 1..contour_count {
            if end_points[i-1] > end_points[i] {
                reject!();
            }
        }
        let point_count = end_points.last().map(|&i| i as usize + 1).unwrap_or(0);

        let instruction_length = try!(Value::read(tape));
        let instructions = read_bytes!(tape, instruction_length);

        let mut flags = Vec::with_capacity(point_count);
        let mut flag_count = 0;
        while flag_count < point_count {
            let flag = try!(u8::read(tape));
            if flag & 0b11000000 > 0 {
                reject!();
            }
            let count = if flag & 0b1000 == 0 { 1 } else { try!(u8::read(tape)) as usize };
            if count == 0 || flag_count + count > point_count {
                reject!();
            }
            for _ in 0..count {
                flags.push(flag);
            }
            flag_count += count;
        }

        macro_rules! read_coordinate(
            ($i:ident, $mask1:expr, $mask2:expr) => ({
                if flags[$i] & $mask1 > 0 {
                    let value = try!(u8::read(tape)) as i16;
                    if flags[$i] & $mask2 > 0 { value } else { -value }
                } else {
                    if flags[$i] & $mask2 > 0 { 0 } else { try!(i16::read(tape)) }
                }
            });
        );
        let mut x = Vec::with_capacity(point_count);
        for i in 0..point_count {
            x.push(read_coordinate!(i, 0b010, 0b010000));
        }
        let mut y = Vec::with_capacity(point_count);
        for i in 0..point_count {
            y.push(read_coordinate!(i, 0b100, 0b100000));
        }

        Ok(Simple {
            end_points: end_points,
            instruction_length: instruction_length,
            instructions: instructions,
            flags: flags,
            x: x,
            y: y,
        })
    }
}
