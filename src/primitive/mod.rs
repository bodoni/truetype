//! Primitive data types.

use std::mem;

use Result;
use tape::{Tape, Value};

/// A fixed-point number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Fixed(pub u32);

/// A font-table tag.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tag(pub u32);

impl From<Fixed> for f32 {
    #[inline]
    fn from(fixed: Fixed) -> Self {
        const SCALE: f32 = 1f32 / (1 << 16) as f32;
        SCALE * (fixed.0 as f32)
    }
}

impl From<Tag> for [u8; 4] {
    #[inline]
    fn from(tag: Tag) -> Self {
        unsafe { mem::transmute(u32::from_be(tag.0)) }
    }
}

impl From<[u8; 4]> for Tag {
    #[inline]
    fn from(bytes: [u8; 4]) -> Self {
        Tag(u32::from_be(unsafe { mem::transmute(bytes) }))
    }
}

impl<'l> From<&'l [u8; 4]> for Tag {
    #[inline(always)]
    fn from(bytes: &'l [u8; 4]) -> Self {
        (*bytes).into()
    }
}

impl From<Fixed> for Tag {
    #[inline(always)]
    fn from(fixed: Fixed) -> Self {
        Tag(fixed.0)
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

implement!(i16, 2);
implement!(u16, 2);
implement!(u32, 4);
implement!(i64, 8);

impl Value for Fixed {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Fixed(try!(Value::read(tape))))
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[test]
    fn from() {
        assert_eq!(Tag::from(b"true"), Tag(0x74727565));
    }
}
