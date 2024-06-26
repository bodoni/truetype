//! The [offset table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff

use crate::tag::Tag;
use crate::Result;

table! {
    @write
    /// An offset table.
    pub Offsets {
        header (Header),

        records (Vec<Record>) |this, tape| {
            tape.take_given(this.header.table_count as usize)
        },
    }
}

table! {
    @write
    /// The header of an offset table.
    #[derive(Copy)]
    pub Header {
        version (u32) |_, tape| { // version
            let value = tape.take()?;
            if !Header::accept(&value) {
                raise!("found an unknown font format");
            }
            Ok(value.into())
        },

        table_count    (u16), // numTables
        search_range   (u16), // searchRange
        entry_selector (u16), // entrySelector
        range_shift    (u16), // rangeShift
    }
}

table! {
    @write
    /// A record of an offset table.
    #[derive(Copy)]
    pub Record {
        tag      (Tag), // tag
        checksum (u32), // checkSum
        offset   (u32), // offset
        size     (u32), // length
    }
}

dereference! { Offsets::records => [Record] }

impl Header {
    /// Check if a tag is recognized.
    #[inline]
    pub fn accept(tag: &Tag) -> bool {
        matches!(&tag.0, [0, 1, 0, 0] | b"OTTO" | b"true" | b"typ1")
    }
}

impl Record {
    /// Compute the checksum of the corresponding table.
    pub fn checksum<T: crate::tape::Read>(&self, tape: &mut T) -> Result<u32> {
        let head = self.tag.0 == *b"head";
        let count = ((self.size + 4 - 1) & !(4 - 1)) / 4;
        let excess = 4 * count - self.size;
        debug_assert!(excess < 4);
        tape.jump(self.offset as u64)?;
        let mut total: u32 = 0;
        for i in 0..count {
            let mut value: u32 = tape.take()?;
            if i + 1 == count {
                value &= !((1u32 << (8 * excess)) - 1);
            }
            if !head || i != 2 {
                total = total.wrapping_add(value);
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Record;
    use crate::Tag;

    macro_rules! ok(($result:expr) => ($result.unwrap()));

    #[test]
    fn record_checksum() {
        macro_rules! checksum(
            ($size:expr, $checksum:expr, $data:expr,) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = Record {
                    tag: Tag(*b"true"),
                    size: $size,
                    offset: 0,
                    checksum: $checksum,
                };
                table.checksum == ok!(table.checksum(&mut reader))
            })
        );
        assert!(!checksum!(
            3 * 4,
            1 + 2 + 4,
            &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3],
        ));
        assert!(checksum!(
            3 * 4,
            1 + 2 + 3,
            &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3],
        ));
    }
}
