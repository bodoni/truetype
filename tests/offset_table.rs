extern crate truetype;

use truetype::Value;

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    use truetype::OffsetTable;

    let mut file = setup!(SourceSerif);
    let OffsetTable { header, records } = ok!(OffsetTable::read(&mut file));
    assert!(header.table_count == 12);
    assert!(header.search_range == 8 * 16);
    assert!(header.entry_selector == 3);
    assert!(header.range_shift == header.table_count * 16 - header.search_range);
    assert!(records.len() == 12);
    for (i, record) in records.iter().enumerate() {
        if i == 6 {
            assert!(ok!(record.checksum(&mut file, |i, chunk| if i == 2 {
                0
            } else {
                chunk
            })));
        } else {
            assert!(ok!(record.checksum(&mut file, |_, chunk| chunk)));
        }
    }
}
