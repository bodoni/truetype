//! The offset table.

use {Result, Tag, Tape, Value, q32};

/// An offset table.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
        tag      (u32), // tag
        checksum (u32), // checkSum
        offset   (u32), // offset
        length   (u32), // length
    }
}

impl Value for OffsetTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        if !is_known(try!(tape.peek::<q32>())) {
            raise!("the font format is not supported");
        }
        let header = read_value!(tape, Header);
        let mut records = vec![];
        for _ in 0..header.table_count {
            records.push(read_value!(tape));
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
                checksum += process(i, read_value!(tape)) as u64;
            }
            Ok(self.checksum == checksum as u32)
        })
    }
}

#[inline]
fn is_known(version: q32) -> bool {
    match version {
        q32(0x00010000) => return true,
        _ => {},
    }
    match &Tag::from(version).into() {
        b"true" | b"typ1" | b"OTTO" => return true,
        _ => {},
    }
    false
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Record;

    #[test]
    fn record_checksum() {
        macro_rules! checksum(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = Record {
                    length: $length,
                    checksum: $checksum,
                    .. Record::default()
                };
                table.checksum(&mut reader, |_, chunk| chunk).unwrap()
            })
        );

        assert!(!checksum!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!( checksum!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
