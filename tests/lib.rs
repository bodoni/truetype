extern crate truetype;

use std::fs::File;
use truetype::{Value, Walue};

mod fixture;

use fixture::Fixture;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (setup(Fixture::$fixture, None));
    ($fixture:ident, $table:expr) => (setup(Fixture::$fixture, Some($table)));
);

#[test]
fn char_mapping_header() {
    use truetype::CharMapping;

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let table = &table.header;
    assert_eq!(table.version, 0);
    assert_eq!(table.table_count, 3);
}

#[test]
fn char_mapping_records() {
    use truetype::CharMapping;

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.records;
    assert_eq!(tables.len(), 3);
    assert_eq!(tables[0].platform_id, 0);
    assert_eq!(tables[0].encoding_id, 3);
    assert_eq!(tables[1].platform_id, 1);
    assert_eq!(tables[1].encoding_id, 0);
    assert_eq!(tables[2].platform_id, 3);
    assert_eq!(tables[2].encoding_id, 1);
}

#[test]
fn char_mapping_encoding_format4() {
    use truetype::char_mapping::{CharMapping, Encoding};

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.encodings;
    assert_eq!(tables.len(), 3);
    match &tables[0] {
        &Encoding::Format4(ref table) => {
            assert_eq!(table.segment_count_x2, 2 * 103);
            assert_eq!(
                table.search_range,
                2 * (1 << 103f64.log2().floor() as usize)
            );
            assert_eq!(table.end_codes.len(), 103);
            assert_eq!(table.start_codes.len(), 103);
            assert_eq!(table.id_deltas.len(), 103);
            assert_eq!(table.id_range_offsets.len(), 103);
            assert_eq!(table.glyph_ids.len(), 353);
            assert_eq!(table.mapping(), fixture::mapping());
        }
        _ => unreachable!(),
    }
    match &tables[2] {
        &Encoding::Format4(ref table) => {
            assert_eq!(table.segment_count_x2, 2 * 103);
        }
        _ => unreachable!(),
    }
}

#[test]
fn char_mapping_encoding_format6() {
    use truetype::char_mapping::{CharMapping, Encoding};

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.encodings;
    assert_eq!(tables.len(), 3);
    match &tables[1] {
        &Encoding::Format6(ref table) => {
            assert_eq!(table.first_code, 9);
            assert_eq!(table.entry_count, 247);
            assert_eq!(table.glyph_ids.len(), 247);
        }
        _ => unreachable!(),
    }
}

#[test]
fn font_header() {
    use truetype::FontHeader;

    let table = ok!(FontHeader::read(&mut setup!(SourceSerif, "head")));
    assert_eq!(format!("{:.3}", f32::from(table.revision)), "1.017");
    assert_eq!(table.units_per_em, 1000);
    assert_eq!(table.mac_style, 0);
    assert_eq!(table.glyph_mapping_format, 0);
}

#[test]
fn glyph_data() {
    use truetype::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};
    use truetype::glyph_data::Description;

    let parameter1 = ok!(FontHeader::read(&mut setup!(OpenSans, "head")));
    let parameter2 = ok!(MaximumProfile::read(&mut setup!(OpenSans, "maxp")));
    let parameter = ok!(GlyphMapping::read(
        &mut setup!(OpenSans, "loca"),
        (&parameter1, &parameter2),
    ));
    let table = ok!(GlyphData::read(&mut setup!(OpenSans, "glyf"), &parameter));
    let glyph = ok!(table[0].as_ref());
    assert_eq!((glyph.min_x, glyph.max_x), (193, 1034));
    assert_eq!((glyph.min_y, glyph.max_y), (0, 1462));
    match glyph.description {
        Description::Simple(ref description) => {
            assert_eq!(&description.x, &[193, 841, 0, -841, 104, 633, 0, -633]);
            assert_eq!(&description.y, &[1462, 0, -1462, 0, 104, 0, 1254, 0])
        }
        _ => unreachable!(),
    }
}

#[test]
fn glyph_mapping() {
    use truetype::{FontHeader, GlyphMapping, MaximumProfile};

    let parameter1 = ok!(FontHeader::read(&mut setup!(OpenSans, "head")));
    let parameter2 = ok!(MaximumProfile::read(&mut setup!(OpenSans, "maxp")));
    match ok!(GlyphMapping::read(
        &mut setup!(OpenSans, "loca"),
        (&parameter1, &parameter2),
    )) {
        GlyphMapping::HalfOffsets(ref offsets) => {
            assert_eq!(
                &offsets[0..10],
                &[0, 27, 27, 27, 27, 73, 102, 189, 293, 403]
            );
        }
        _ => unreachable!(),
    }
}

#[test]
fn horizontal_header() {
    use truetype::HorizontalHeader;

    let table = ok!(HorizontalHeader::read(&mut setup!(SourceSerif, "hhea")));
    assert_eq!(table.ascender, 918);
    assert_eq!(table.descender, -335);
    assert_eq!(table.horizontal_metric_count, 547);
}

#[test]
fn horizontal_metrics() {
    use truetype::{HorizontalHeader, HorizontalMetrics, MaximumProfile};

    let parameter1 = ok!(HorizontalHeader::read(&mut setup!(SourceSerif, "hhea")));
    let parameter2 = ok!(MaximumProfile::read(&mut setup!(SourceSerif, "maxp")));
    let table = ok!(HorizontalMetrics::read(
        &mut setup!(SourceSerif, "hmtx"),
        (&parameter1, &parameter2),
    ));
    assert_eq!(table.records.len(), 547);
    assert_eq!(table.left_side_bearings.len(), 547 - 547);
    assert_eq!(table.get(42), (549, 45));
}

#[test]
fn maximum_profile() {
    use truetype::MaximumProfile;

    match ok!(MaximumProfile::read(&mut setup!(SourceSerif, "maxp"))) {
        MaximumProfile::Version0(ref table) => {
            assert_eq!(table.glyph_count, 547);
        }
        _ => unreachable!(),
    }
}

#[test]
fn naming_table() {
    use truetype::NamingTable;

    match ok!(NamingTable::read(&mut setup!(SourceSerif, "name"))) {
        NamingTable::Format0(ref table) => {
            assert_eq!(table.count, 26);
            assert_eq!(ok!(table.strings())[9], "Frank Grießhammer");
        }
        _ => unreachable!(),
    }
}

#[test]
fn offset_table() {
    use truetype::OffsetTable;

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
    for (i, record) in records.iter().enumerate() {
        if i == 6 {
            assert!(ok!(
                record.checksum(&mut file, |i, chunk| if i == 2 { 0 } else { chunk })
            ));
        } else {
            assert!(ok!(record.checksum(&mut file, |_, chunk| chunk)));
        }
    }
}

#[test]
fn postscript() {
    use truetype::PostScript;

    match ok!(PostScript::read(&mut setup!(OpenSans, "post"))) {
        PostScript::Version2(ref table) => {
            assert_eq!(table.glyph_count, 938);
            assert_eq!(table.glyph_names.len(), 938);
            assert_eq!(&table.glyph_names[0], ".notdef");
            assert_eq!(&table.glyph_names[42], "G");
        }
        _ => unreachable!(),
    }
    match ok!(PostScript::read(&mut setup!(SourceSerif, "post"))) {
        PostScript::Version3(ref table) => {
            assert_eq!(f32::from(table.version), 3.0);
            assert_eq!(table.underline_position, -75);
        }
        _ => unreachable!(),
    }
}

#[test]
fn windows_metrics() {
    use truetype::WindowsMetrics;

    match ok!(WindowsMetrics::read(&mut setup!(SourceSerif, "OS/2"))) {
        WindowsMetrics::Version3(ref table) => {
            assert_eq!(table.panose, [2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
            assert_eq!(stringify(&table.vendor_id), "ADBE");
            assert_eq!(table.break_char, 32);
        }
        _ => unreachable!(),
    }
}

fn setup(fixture: Fixture, table: Option<&str>) -> File {
    use std::io::{Seek, SeekFrom};

    let mut file = ok!(File::open(fixture.path()));
    ok!(file.seek(SeekFrom::Start(
        table.map(|table| fixture.offset(table)).unwrap_or(0),
    )));
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
