extern crate truetype;

use std::fs::File;
use truetype::Value;
use truetype::compound::OffsetTable;

mod fixture;

#[test]
fn char_mapping_encodings() {
    use truetype::compound::{CharMapping, CharMappingEncoding};

    let mut file = setup();
    let offset = OffsetTable::read(&mut file).unwrap();
    let mapping = CharMapping::read(&mut file, &offset.records[5]).unwrap();
    let tables = &mapping.encodings;

    assert_eq!(tables.len(), 3);
    match &tables[0] {
        &CharMappingEncoding::Format4(ref table) => {
            assert_eq!(table.segCountX2, 2 * 103);
            assert_eq!(table.searchRange, 2 * (1 << 103f64.log2().floor() as usize));
            assert_eq!(table.endCode.len(), 103);
            assert_eq!(table.startCode.len(), 103);
            assert_eq!(table.idDelta.len(), 103);
            assert_eq!(table.idRangeOffset.len(), 103);
            assert_eq!(table.glyphIdArray.len(), 353);
            assert_eq!(table.mapping(), fixture::mapping());
        },
        _ => unreachable!(),
    }
    match &tables[1] {
        &CharMappingEncoding::Format6(ref table) => {
            assert_eq!(table.firstCode, 9);
            assert_eq!(table.entryCount, 247);
            assert_eq!(table.glyphIdArray.len(), 247);
        },
        _ => unreachable!(),
    }
    match &tables[2] {
        &CharMappingEncoding::Format4(ref table) => {
            assert_eq!(table.segCountX2, 2 * 103);
        },
        _ => unreachable!(),
    }
}

#[test]
fn char_mapping_header() {
    use truetype::compound::CharMapping;

    let mut file = setup();
    let offset = OffsetTable::read(&mut file).unwrap();
    let mapping = CharMapping::read(&mut file, &offset.records[5]).unwrap();
    let table = &mapping.header;

    assert_eq!(table.version, 0);
    assert_eq!(table.numTables, 3);
}

#[test]
fn char_mapping_records() {
    use truetype::compound::CharMapping;

    let mut file = setup();
    let offset = OffsetTable::read(&mut file).unwrap();
    let mapping = CharMapping::read(&mut file, &offset.records[5]).unwrap();
    let tables = &mapping.records;

    assert_eq!(tables.len(), 3);
    assert_eq!(tables[0].platformID, 0);
    assert_eq!(tables[0].encodingID, 3);
    assert_eq!(tables[1].platformID, 1);
    assert_eq!(tables[1].encodingID, 0);
    assert_eq!(tables[2].platformID, 3);
    assert_eq!(tables[2].encodingID, 1);
}

#[test]
fn offset_table() {
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
