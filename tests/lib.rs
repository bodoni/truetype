extern crate truetype;

use std::fs::File;
use truetype::Value;

mod fixture;

#[test]
fn char_mapping_encodings() {
    use truetype::compound::{CharMapping, CharMappingEncoding};

    let mut file = setup(15620);
    let mapping = CharMapping::read(&mut file).unwrap();
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

    let mut file = setup(15620);
    let mapping = CharMapping::read(&mut file).unwrap();
    let table = &mapping.header;

    assert_eq!(table.version, 0);
    assert_eq!(table.numTables, 3);
}

#[test]
fn char_mapping_records() {
    use truetype::compound::CharMapping;

    let mut file = setup(15620);
    let mapping = CharMapping::read(&mut file).unwrap();
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
fn font_header() {
    use truetype::compound::FontHeader;

    let mut file = setup(204);
    let table = FontHeader::read(&mut file).unwrap();

    assert_eq!(format!("{:.3}", table.fontRevision.as_f32()), "1.017");
    assert_eq!(table.unitsPerEm, 1000);
    assert_eq!(table.macStyle, 0);
}

#[test]
fn maximum_profile() {
    use truetype::compound::MaximumProfile;

    let mut file = setup(296);
    let table = MaximumProfile::read(&mut file).unwrap();

    match table {
        MaximumProfile::Version05(ref table) => {
            assert_eq!(table.numGlyphs, 547);
        },
        _ => unreachable!(),
    }
}

#[test]
fn naming_table() {
    use truetype::compound::NamingTable;

    let mut file = setup(400);
    let table = NamingTable::read(&mut file).unwrap();

    match table {
        NamingTable::Format0(ref table) => {
            assert_eq!(table.count, 26);
            assert_eq!(table.strings().unwrap()[9], "Frank Grießhammer");
        },
        _ => unreachable!(),
    }
}

#[test]
fn offset_table() {
    use truetype::compound::OffsetTable;

    let mut file = setup(0);
    let OffsetTable { header, records } = OffsetTable::read(&mut file).unwrap();

    assert_eq!(header.numTables, 12);
    assert_eq!(records.len(), 12);

    assert!(records[5].checksum(&mut file, |_, chunk| chunk).unwrap());
    assert!(records[6].checksum(&mut file, |i, chunk| if i == 2 { 0 } else { chunk }).unwrap());
}

#[test]
fn postscript() {
    use truetype::compound::PostScript;

    let mut file = setup(17700);
    let table = PostScript::read(&mut file).unwrap();

    match table {
        PostScript::Version30(ref table) => {
            assert_eq!(table.version.as_f32(), 3.0);
            assert_eq!(table.underlinePosition, -75);
        },
        _ => unreachable!(),
    }
}

fn setup(offset: u64) -> File {
    use std::fs;
    use std::io::{Seek, SeekFrom};
    use std::path::PathBuf;

    let path = PathBuf::from("tests/fixtures/SourceSerifPro-Regular.otf");
    assert!(fs::metadata(&path).is_ok());
    let mut file = File::open(&path).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();
    file
}
