#[macro_use]
mod support;

use std::io::{Cursor, Seek, SeekFrom};

use truetype::tables::Offsets;
use truetype::value::{Read, Write};

#[test]
fn read() {
    let mut file = setup!(SourceSerif);
    let offsets = ok!(Offsets::read(&mut file));
    test(&offsets, Some(&mut file));
}

#[test]
fn write() {
    let mut file = setup!(SourceSerif);
    let offsets = ok!(Offsets::read(&mut file));

    let mut cursor = Cursor::new(Vec::new());
    ok!(offsets.write(&mut cursor));
    ok!(cursor.seek(SeekFrom::Start(0)));

    let offsets = ok!(Offsets::read(&mut cursor));
    test::<Cursor<Vec<_>>>(&offsets, None);
}

fn test<T>(Offsets { header, records }: &Offsets, tape: Option<&mut T>)
where
    T: truetype::tape::Read,
{
    assert_eq!(header.table_count, 12);
    assert_eq!(header.search_range, 8 * 16);
    assert_eq!(header.entry_selector, 3);
    assert_eq!(
        header.range_shift,
        header.table_count * 16 - header.search_range
    );
    assert_eq!(records.len(), 12);
    if let Some(tape) = tape {
        let (_, failures): (Vec<_>, Vec<_>) = records
            .iter()
            .map(|record| (record, ok!(record.checksum(tape))))
            .partition(|(record, checksum)| record.checksum == *checksum);
        assert_eq!(failures.len(), 0);
    }
}
