#[macro_use]
mod support;

use truetype::{OffsetTable, Value};

#[test]
fn read() {
    let mut file = setup!(SourceSerif);
    let OffsetTable { header, records } = ok!(OffsetTable::read(&mut file));
    assert_eq!(header.table_count, 12);
    assert_eq!(header.search_range, 8 * 16);
    assert_eq!(header.entry_selector, 3);
    assert_eq!(
        header.range_shift,
        header.table_count * 16 - header.search_range
    );
    assert_eq!(records.len(), 12);
    let (_, failures): (Vec<_>, Vec<_>) = records
        .iter()
        .map(|record| (record, ok!(record.checksum(&mut file))))
        .partition(|(record, checksum)| record.checksum == *checksum);
    assert_eq!(failures.len(), 0, "{:?}", failures);
}
