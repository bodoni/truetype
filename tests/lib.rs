extern crate truetype;

use std::fs::File;
use truetype::Value;

mod fixture;

#[test]
fn char_mapping_encodings() {
    use truetype::{CharMapping, CharMappingEncoding};

    let mapping = CharMapping::read(&mut setup(15620)).unwrap();
    let tables = &mapping.encodings;
    assert_eq!(tables.len(), 3);
    match &tables[0] {
        &CharMappingEncoding::Format4(ref table) => {
            assert_eq!(table.segment_count_x2, 2 * 103);
            assert_eq!(table.search_range, 2 * (1 << 103f64.log2().floor() as usize));
            assert_eq!(table.end_codes.len(), 103);
            assert_eq!(table.start_codes.len(), 103);
            assert_eq!(table.id_deltas.len(), 103);
            assert_eq!(table.id_range_offsets.len(), 103);
            assert_eq!(table.glyph_indices.len(), 353);
            assert_eq!(table.mapping(), fixture::mapping());
        },
        _ => unreachable!(),
    }
    match &tables[1] {
        &CharMappingEncoding::Format6(ref table) => {
            assert_eq!(table.first_code, 9);
            assert_eq!(table.entry_count, 247);
            assert_eq!(table.glyph_indices.len(), 247);
        },
        _ => unreachable!(),
    }
    match &tables[2] {
        &CharMappingEncoding::Format4(ref table) => {
            assert_eq!(table.segment_count_x2, 2 * 103);
        },
        _ => unreachable!(),
    }
}

#[test]
fn char_mapping_header() {
    use truetype::CharMapping;

    let mapping = CharMapping::read(&mut setup(15620)).unwrap();
    let table = &mapping.header;
    assert_eq!(table.version, 0);
    assert_eq!(table.table_count, 3);
}

#[test]
fn char_mapping_records() {
    use truetype::CharMapping;

    let mapping = CharMapping::read(&mut setup(15620)).unwrap();
    let tables = &mapping.records;
    assert_eq!(tables.len(), 3);
    assert_eq!(tables[0].platform_id, 0);
    assert_eq!(tables[0].encoding_id, 3);
    assert_eq!(tables[1].platform_id, 1);
    assert_eq!(tables[1].encoding_id, 0);
    assert_eq!(tables[2].platform_id, 3);
    assert_eq!(tables[2].encoding_id, 1);
}

#[test]
fn font_header() {
    use truetype::FontHeader;

    let table = FontHeader::read(&mut setup(204)).unwrap();
    assert_eq!(format!("{:.3}", f32::from(table.font_revision)), "1.017");
    assert_eq!(table.units_per_em, 1000);
    assert_eq!(table.mac_style, 0);
}

#[test]
fn horizontal_header() {
    use truetype::HorizontalHeader;

    let table = HorizontalHeader::read(&mut setup(260)).unwrap();
    assert_eq!(table.ascender, 918);
    assert_eq!(table.descender, -335);
    assert_eq!(table.horizontal_metric_count, 547);
}

#[test]
fn horizontal_metrics() {
    use truetype::{HorizontalHeader, HorizontalMetrics, MaximumProfile};

    let header = HorizontalHeader::read(&mut setup(260)).unwrap();
    let profile = MaximumProfile::read(&mut setup(296)).unwrap();
    let table = HorizontalMetrics::read(&mut setup(55460), &header, &profile).unwrap();
    assert_eq!(table.metrics.len(), 547);
    assert_eq!(table.left_side_bearings.len(), 547 - 547);
}

#[test]
fn maximum_profile() {
    use truetype::MaximumProfile;

    let table = MaximumProfile::read(&mut setup(296)).unwrap();
    match table {
        MaximumProfile::Version05(ref table) => {
            assert_eq!(table.glyph_count, 547);
        },
        _ => unreachable!(),
    }
}

#[test]
fn naming_table() {
    use truetype::NamingTable;

    let table = NamingTable::read(&mut setup(400)).unwrap();
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
    use truetype::OffsetTable;

    let mut file = setup(0);
    let OffsetTable { header, records } = OffsetTable::read(&mut file).unwrap();

    assert_eq!(header.table_count, 12);
    assert_eq!(header.search_range, 8 * 16);
    assert_eq!(header.entry_selector, 3);
    assert_eq!(header.range_shift, header.table_count * 16 - header.search_range);

    assert_eq!(records.len(), 12);
    for (i, record) in records.iter().enumerate() {
        assert!(if i == 6 {
            record.checksum(&mut file, |i, chunk| if i == 2 { 0 } else { chunk })
        } else {
            record.checksum(&mut file, |_, chunk| chunk)
        }.unwrap());
    }
}

#[test]
fn postscript() {
    use truetype::PostScriptInfo;

    let mut file = setup(17700);
    let table = PostScriptInfo::read(&mut file).unwrap();

    match table {
        PostScriptInfo::Version30(ref table) => {
            assert_eq!(f32::from(table.version), 3.0);
            assert_eq!(table.underlinePosition, -75);
        },
        _ => unreachable!(),
    }
}

#[test]
fn windows_metrics() {
    use truetype::WindowsMetrics;

    let table = WindowsMetrics::read(&mut setup(304)).unwrap();
    match table {
        WindowsMetrics::Version3(ref table) => {
            assert_eq!(table.panose, &[2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
            assert_eq!(stringify(&table.achVendID), "ADBE");
            assert_eq!(table.usBreakChar, 32);
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

fn stringify<T>(data: &[T]) -> &str {
    use std::{mem, slice, str};
    unsafe {
        let length = data.len() * mem::size_of::<T>();
        let bytes = slice::from_raw_parts(data as *const _ as *const _, length);
        str::from_utf8_unchecked(bytes)
    }
}
