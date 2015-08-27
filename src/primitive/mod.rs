//! Primitive data types.

use std::mem;

use Result;
use tape::{Tape, Value};

/// A fixed-point number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

impl Fixed {
    /// Return as `f32`.
    pub fn as_f32(&self) -> f32 {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (self.0 as f32)
    }
}

macro_rules! fill(
    ($tape:ident, $count:expr, $buffer:ident) => (
        if try!(::std::io::Read::read($tape, &mut $buffer)) != $count {
            return raise!("failed to read as much as needed");
        }
    );
);

macro_rules! read(
    ($tape:ident, $size:expr) => (unsafe {
        let mut buffer: [u8; $size] = mem::uninitialized();
        fill!($tape, $size, buffer);
        mem::transmute(buffer)
    });
);

macro_rules! implement {
    ($name:ident, 1) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok(read!(tape, 1))
        }
    });
    ($name:ident, $size:expr) => (impl Value for $name {
        fn read<T: Tape>(tape: &mut T) -> Result<Self> {
            Ok($name::from_be(read!(tape, $size)))
        }
    });
}

implement!(u16, 2);
implement!(u32, 4);

impl Value for Fixed {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Fixed(try!(Value::read(tape))))
    }
}
