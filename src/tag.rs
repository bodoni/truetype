use std::fmt;

use crate::{Result, Tape, Value};

/// A tag.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Tag(pub [u8; 4]);

dereference! { Tag::0 => [u8; 4] }

impl fmt::Debug for Tag {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        match str::from_utf8(&self.0[..]) {
            Ok(value) if value.chars().all(char::is_alphanumeric) => {
                write!(formatter, "Tag({value})")
            }
            _ => write!(formatter, "Tag(0x{:08X})", u32::from(*self)),
        }
    }
}

impl From<u32> for Tag {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Tag(u32::from_be(value).to_ne_bytes())
    }
}

impl From<Tag> for u32 {
    #[inline(always)]
    fn from(tag: Tag) -> Self {
        u32::from_be_bytes(tag.0)
    }
}

impl Value for Tag {
    #[inline(always)]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(Tag(tape.take()?))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Tag;
    use crate::Value;

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Tag(*b"true")), r#"Tag(true)"#);
        assert_eq!(format!("{:?}", Tag([0, 1, 0, 0])), r#"Tag(0x00010000)"#);
    }

    #[test]
    fn from() {
        assert_eq!(Tag(*b"true"), Tag::from(0x74727565));
    }

    #[test]
    fn into() {
        assert_eq!(u32::from(Tag(*b"true")), 0x74727565);
    }

    #[test]
    fn read() {
        let mut tape = Cursor::new(b"true".to_vec());
        assert_eq!(Tag::read(&mut tape).unwrap(), Tag(*b"true"));
    }
}
