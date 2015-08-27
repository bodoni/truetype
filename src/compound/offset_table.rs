use std::mem;

use Result;
use tape::{Tape, Value};
use primitive::Fixed;

/// The offset table.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OffsetTable {
    pub header: OffsetTableHeader,
    pub records: Vec<OffsetTableRecord>,
}

table! {
    #[doc = "The header of the offset table."]
    #[derive(Copy)]
    pub OffsetTableHeader {
        version       (Fixed),
        numTables     (u16  ),
        searchRange   (u16  ),
        entrySelector (u16  ),
        rangeShift    (u16  ),
    }
}

table! {
    #[doc = "A record of the offset table."]
    #[derive(Copy)]
    pub OffsetTableRecord {
        tag      (u32),
        checkSum (u32),
        offset   (u32),
        length   (u32),
    }
}

impl Value for OffsetTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        match try!(tape.peek::<Fixed>()) {
            Fixed(0x00010000) => {},
            version => match &tag!(version) {
                b"true" | b"typ1" | b"OTTO" => {},
                _ => raise!("the format is not supported"),
            }
        }
        let header = try!(OffsetTableHeader::read(tape));
        let mut records = vec![];
        for _ in 0..header.numTables {
            records.push(try!(OffsetTableRecord::read(tape)));
        }
        Ok(OffsetTable { header: header, records: records })
    }
}

impl OffsetTableRecord {
    #[doc(hidden)]
    pub fn check<T, F>(&self, tape: &mut T, process: F) -> Result<bool>
        where T: Tape, F: Fn(usize, u32) -> u32
    {
        let length = {
            let size = mem::size_of::<u32>();
            ((self.length as usize + size - 1) & !(size - 1)) / size
        };
        tape.stay(|tape| {
            try!(tape.jump(self.offset as u64));
            let mut checksum: u64 = 0;
            for i in 0..length {
                checksum += process(i, try!(Value::read(tape))) as u64;
            }
            Ok(self.checkSum == checksum as u32)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::OffsetTableRecord;

    #[test]
    fn record_check() {
        macro_rules! check(
            ($length:expr, $checksum:expr, $data:expr) => ({
                let data: &[u8] = $data;
                let mut reader = Cursor::new(data);
                let table = OffsetTableRecord {
                    length: $length,
                    checkSum: $checksum,
                    .. OffsetTableRecord::default()
                };
                table.check(&mut reader, |_, chunk| chunk).unwrap()
            })
        );

        assert!(!check!(3 * 4, 1 + 2 + 4, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
        assert!( check!(3 * 4, 1 + 2 + 3, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]));
    }
}
