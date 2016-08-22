//! The [offset table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/otff.htm

use {Result, Tag, Tape, Value, q32};

/// An offset table.
#[derive(Clone, Debug)]
pub struct OffsetTable {
    pub header: Header,
    pub records: Vec<Record>,
}

table! {
    #[doc = "The header of an offset table."]
    #[derive(Copy)]
    pub Header {
        version        (q32), // version
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

impl Value for OffsetTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let header = try!(tape.take::<Header>());
        match header.version {
            q32(0x00010000) => {},
            version => match &*Tag::from(version) {
                b"true" | b"typ1" | b"OTTO" => {},
                _ => raise!("found an unknown font format"),
            },
        }
        let mut records = vec![];
        for _ in 0..header.table_count {
            records.push(try!(tape.take()));
        }
        Ok(OffsetTable { header: header, records: records })
    }
}

impl Record {
    /// Compute the checksum of the corresponding table and compare it with the
    /// one in the record.
    pub fn checksum<T, F>(&self, tape: &mut T, process: F) -> Result<bool>
        where T: Tape, F: Fn(usize, u32) -> u32
    {
        let length = ((self.length as usize + 4 - 1) & !(4 - 1)) / 4;
        tape.stay(|tape| {
            try!(tape.jump(self.offset as u64));
            let mut checksum: u64 = 0;
            for i in 0..length {
                checksum += process(i, try!(tape.take())) as u64;
            }
            Ok(self.checksum == checksum as u32)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use Tag;
    use super::Record;

    #[test]
    fn record_checksum() {
        macro_rules! checksum(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = Record {
                    tag: Tag(*b"true"),
                    length: $length,
                    offset: 0,
                    checksum: $checksum,
                };
                table.checksum(&mut reader, |_, chunk| chunk).unwrap()
            })
        );
        assert!(!checksum!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!(checksum!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
