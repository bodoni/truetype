extern crate truetype;

use std::fs::File;
use truetype::Value;

#[test]
fn offset_table() {
    use truetype::compound::OffsetTable;

    let mut file = setup();
    let OffsetTable { header, records } = OffsetTable::read(&mut file).unwrap();

    assert_eq!(header.numTables, 12);
    assert_eq!(records.len(), 12);
}

fn setup() -> File {
    use std::fs;
    use std::path::PathBuf;

    let path = PathBuf::from("tests/fixtures/SourceSerifPro-Regular.otf");
    assert!(fs::metadata(&path).is_ok());
    File::open(&path).unwrap()
}
