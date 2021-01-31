use std::fmt;
use std::ops::Deref;

use crate::{Result, Tape, Value};

/// A tag.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Tag(pub [u8; 4]);

impl Deref for Tag {
    type Target = [u8; 4];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        match str::from_utf8(&self.0[..]) {
            Ok(name) => write!(formatter, "Tag({:?})", name),
            _ => write!(formatter, "Tag({:?})", self.0),
        }
    }
}

impl From<u32> for Tag {
    #[inline(always)]
    fn from(number: u32) -> Self {
        use std::mem;

        Tag(unsafe { mem::transmute(u32::from_be(number)) })
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

    use crate::Value;
    use super::Tag;

    #[test]
    fn debug() {
        assert!(format!("{:?}", Tag(*b"true")) == r#"Tag("true")"#);
    }

    #[test]
    fn from() {
        assert!(Tag(*b"true") == Tag::from(0x74727565));
    }

    #[test]
    fn read() {
        let mut cursor = Cursor::new(b"true".to_vec());
        assert!(Tag::read(&mut cursor).unwrap() == Tag(*b"true"));
    }
}
