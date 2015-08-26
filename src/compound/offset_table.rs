use std::mem;

use Result;
use band::{Band, Value};
use primitive::{Fixed, ULong, UShort};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OffsetTable {
    pub header: OffsetTableHeader,
    pub records: Vec<OffsetTableRecord>,
}

table! {
    #[derive(Copy)]
    pub OffsetTableHeader {
        version       (Fixed ),
        numTables     (UShort),
        searchRange   (UShort),
        entrySelector (UShort),
        rangeShift    (UShort),
    }
}

table! {
    #[derive(Copy)]
    pub OffsetTableRecord {
        tag      (ULong),
        checkSum (ULong),
        offset   (ULong),
        length   (ULong),
    }
}

impl Value for OffsetTable {
    fn read<T: Band>(band: &mut T) -> Result<Self> {
        let header = match &tag!(try!(band.peek::<Fixed>())) {
            b"OTTO" => try!(OffsetTableHeader::read(band)),
            _ => raise!("the format of a font is not supported"),
        };
        let mut records = vec![];
        for _ in 0..header.numTables {
            records.push(try!(OffsetTableRecord::read(band)));
        }
        Ok(OffsetTable { header: header, records: records })
    }
}

impl OffsetTableRecord {
    #[doc(hidden)]
    pub fn check<T, F>(&self, band: &mut T, process: F) -> Result<bool>
        where T: Band, F: Fn(usize, ULong) -> ULong
    {
        let length = {
            let size = mem::size_of::<ULong>();
            ((self.length as usize + size - 1) & !(size - 1)) / size
        };
        band.stay(|band| {
            try!(band.jump(self.offset as u64));
            let mut checksum: u64 = 0;
            for i in 0..length {
                checksum += process(i, try!(Value::read(band))) as u64;
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
