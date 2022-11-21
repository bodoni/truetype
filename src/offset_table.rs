//! The [offset table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff

use crate::{Result, Tag, Tape};

table! {
    #[doc = "An offset table."]
    pub OffsetTable {
        header (Header),

        records (Vec<Record>) |this, tape| {
            tape.take_given(this.header.table_count as usize)
        },
    }
}

table! {
    #[doc = "The header of an offset table."]
    #[derive(Copy)]
    pub Header {
        version (u32) |_, tape| { // version
            let value = tape.take()?;
            match value {
                0x00010000 => {},
                value => match &*Tag::from(value) {
                    b"OTTO" | b"true" | b"typ1" => {},
                    _ => raise!("found an unknown font format"),
                },
            }
            Ok(value)
        },

        table_count    (u16), // numTables
        search_range   (u16), // searchRange
        entry_selector (u16), // entrySelector
        range_shift    (u16), // rangeShift
    }
}

table! {
    #[doc = "A record of an offset table."]
    #[derive(Copy)]
    pub Record {
        tag      (Tag), // tag
        checksum (u32), // checkSum
        offset   (u32), // offset
        length   (u32), // length
    }
}

impl Record {
    /// Compute the checksum of the corresponding table.
    pub fn checksum<T: Tape>(&self, tape: &mut T) -> Result<u32> {
        let head = self.tag == Tag(*b"head");
        let length = ((self.length as usize + 4 - 1) & !(4 - 1)) / 4;
        tape.stay(|tape| {
            tape.jump(self.offset as u64)?;
            let mut total: u32 = 0;
            for i in 0..length {
                let value: u32 = tape.take()?;
                if !head || i != 2 {
                    total = total.wrapping_add(value);
                }
            }
            Ok(total)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Record;
    use crate::Tag;

    #[test]
    fn record_checksum() {
        macro_rules! checksum(
            ($length:expr, $checksum:expr, $data:expr,) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = Record {
                    tag: Tag(*b"true"),
                    length: $length,
                    offset: 0,
                    checksum: $checksum,
                };
                table.checksum == table.checksum(&mut reader).unwrap()
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
