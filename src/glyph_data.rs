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
    Compound(Compound),
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
    #[doc = "A compound-glyph description."]
    pub Compound {
        flags (u16), // flags
        index (u16), // glyphIndex

        arguments (Arguments) |tape, this| { // argument1, argument2
            Walue::read(tape, this.flags)
        },

        options (Options) |tape, this| {
            Walue::read(tape, this.flags)
        },
    }
}

/// Arguments of a compound glyph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arguments {
    Int8([i8; 2]),
    Int16([i16; 2]),
    UInt8([u8; 2]),
    UInt16([u16; 2]),
}

/// Options of a compound glyph.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Options {
    None,
    Type1,
    Type2,
    Type3,
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
            Ok(Description::Compound(try!(Value::read(tape))))
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
            let flag = try!(flags::Simple::read(tape));
            if flag.is_invalid() {
                reject!();
            }
            let count = if flag.is_repeated() { try!(u8::read(tape)) as usize } else { 1 };
            if count == 0 || flag_count + count > point_count {
                reject!();
            }
            for _ in 0..count {
                flags.push(flag.into());
            }
            flag_count += count;
        }

        let mut x = Vec::with_capacity(point_count);
        for i in 0..point_count {
            let flag = flags::Simple(flags[i]);
            if flag.is_x_short() {
                let value = try!(u8::read(tape)) as i16;
                x.push(if flag.is_x_positive() { value } else { -value });
            } else {
                x.push(if flag.is_x_same() { 0 } else { try!(i16::read(tape)) });
            }
        }

        let mut y = Vec::with_capacity(point_count);
        for i in 0..point_count {
            let flag = flags::Simple(flags[i]);
            if flag.is_y_short() {
                let value = try!(u8::read(tape)) as i16;
                y.push(if flag.is_y_positive() { value } else { -value });
            } else {
                y.push(if flag.is_y_same() { 0 } else { try!(i16::read(tape)) });
            }
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

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Options::None
    }
}

impl Walue<u16> for Options {
    fn read<T: Tape>(_: &mut T, flags: u16) -> Result<Self> {
        let flags = flags::Compound(flags);
        if !flags.has_options() {
            return Ok(Options::None);
        }
        unreachable!()
    }
}

mod flags {
    macro_rules! flags {
        (pub $structure:ident($kind:ident) {
            $($mask:expr => $name:ident,)*
        }) => (
            #[derive(Clone, Copy)]
            pub struct $structure(pub $kind);

            impl $structure {
                $(
                    #[inline(always)]
                    pub fn $name(&self) -> bool {
                        self.0 & $mask > 0
                    }
                )*
            }

            impl ::Value for $structure {
                #[inline(always)]
                fn read<T: ::Tape>(tape: &mut T) -> ::Result<Self> {
                    Ok($structure(try!($kind::read(tape))))
                }
            }

            impl From<$structure> for $kind {
                #[inline(always)]
                fn from(flags: $structure) -> $kind {
                    flags.0
                }
            }
        );
    }

    flags! {
        pub Simple(u8) {
            0b0000_0010 => is_x_short,
            0b0000_0100 => is_y_short,
            0b0000_1000 => is_repeated,
            0b0001_0000 => is_x_positive,
            0b0001_0000 => is_x_same,
            0b0010_0000 => is_y_positive,
            0b0010_0000 => is_y_same,
            0b1100_0000 => is_invalid,
        }
    }

    flags! {
        pub Compound(u16) {
            0b0000_0000_0000_1000 => has_options,
        }
    }
}
