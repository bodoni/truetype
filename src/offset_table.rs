use {Fixed, Result, Tag, Tape, Value};

/// An offset table.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OffsetTable {
    pub header: OffsetTableHeader,
    pub records: Vec<OffsetTableRecord>,
}

table! {
    #[doc = "The header of an offset table."]
    #[derive(Copy)]
    pub OffsetTableHeader {
        version        (Fixed),
        table_count    (u16  ), // numTables
        search_range   (u16  ), // searchRange
        entry_selector (u16  ), // entrySelector
        range_shift    (u16  ), // rangeShift
    }
}

table! {
    #[doc = "A record of an offset table."]
    #[derive(Copy)]
    pub OffsetTableRecord {
        tag      (u32),
        checksum (u32), // checkSum
        offset   (u32),
        length   (u32),
    }
}

impl Value for OffsetTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        if !is_known(try!(tape.peek::<Fixed>())) {
            raise!("the font format is not supported");
        }
        let header = try!(OffsetTableHeader::read(tape));
        let mut records = vec![];
        for _ in 0..header.table_count {
            records.push(try!(OffsetTableRecord::read(tape)));
        }
        Ok(OffsetTable { header: header, records: records })
    }
}

impl OffsetTableRecord {
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
                checksum += process(i, try!(Value::read(tape))) as u64;
            }
            Ok(self.checksum == checksum as u32)
        })
    }
}

#[inline]
fn is_known(version: Fixed) -> bool {
    match version {
        Fixed(0x00010000) => return true,
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
    use super::OffsetTableRecord;

    #[test]
    fn record_checksum() {
        macro_rules! checksum(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = OffsetTableRecord {
                    length: $length,
                    checksum: $checksum,
                    .. OffsetTableRecord::default()
                };
                table.checksum(&mut reader, |_, chunk| chunk).unwrap()
            })
        );

        assert!(!checksum!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!( checksum!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
