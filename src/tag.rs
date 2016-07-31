use std::mem;

use {Result, Tape, Value, q32};

/// A table tag.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Tag(pub [u8; 4]);

deref! { Tag::0 => [u8; 4] }

impl From<q32> for Tag {
    #[inline(always)]
    fn from(q32(number): q32) -> Self {
        Tag(unsafe { mem::transmute(u32::from_be(number)) })
    }
}

impl Value for Tag {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Tag(try!(tape.take())))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use {Value, q32};
    use super::Tag;

    #[test]
    fn from() {
        assert_eq!(Tag(*b"true"), Tag::from(q32(0x74727565)));
    }

    #[test]
    fn read() {
        let mut cursor = Cursor::new(b"true".to_vec());
        assert_eq!(Tag::read(&mut cursor).unwrap(), Tag(*b"true"));
    }
}
