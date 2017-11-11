extern crate truetype;

use std::fs::File;
use truetype::{Value, Walue};

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (setup(Fixture::$fixture, None));
    ($fixture:ident, $table:expr) => (setup(Fixture::$fixture, Some($table)));
);

mod fixture;

use fixture::Fixture;

#[test]
fn char_mapping_header() {
    use truetype::CharMapping;

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let table = &table.header;
    assert!(table.version == 0);
    assert!(table.table_count == 3);
}

#[test]
fn char_mapping_records() {
    use truetype::CharMapping;

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.records;
    assert!(tables.len() == 3);
    assert!(tables[0].platform_id == 0);
    assert!(tables[0].encoding_id == 3);
    assert!(tables[1].platform_id == 1);
    assert!(tables[1].encoding_id == 0);
    assert!(tables[2].platform_id == 3);
    assert!(tables[2].encoding_id == 1);
}

#[test]
fn char_mapping_encoding_format4() {
    use truetype::char_mapping::{CharMapping, Encoding, Mapping};

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.encodings;
    assert!(tables.len() == 3);
    match &tables[0] {
        &Encoding::Format4(ref table) => {
            assert!(table.segment_count_x2 == 2 * 103);
            assert!(table.search_range == 2 * (1 << 103f64.log2().floor() as usize));
            assert!(table.end_codes.len() == 103);
            assert!(table.start_codes.len() == 103);
            assert!(table.id_deltas.len() == 103);
            assert!(table.id_range_offsets.len() == 103);
            assert!(table.glyph_ids.len() == 353);
            if let Mapping::U16(ref mapping) = Fixture::SourceSerif.mappings()[0] {
                assert!(&table.mapping() == mapping);
            } else {
                unreachable!();
            }
        }
        _ => unreachable!(),
    }
    match &tables[2] {
        &Encoding::Format4(ref table) => {
            assert!(table.segment_count_x2 == 2 * 103);
        }
        _ => unreachable!(),
    }
}

#[test]
fn char_mapping_encoding_format6() {
    use truetype::char_mapping::{CharMapping, Encoding};

    let table = ok!(CharMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.encodings;
    assert!(tables.len() == 3);
    match &tables[1] {
        &Encoding::Format6(ref table) => {
            assert!(table.first_code == 9);
            assert!(table.entry_count == 247);
            assert!(table.glyph_ids.len() == 247);
        }
        _ => unreachable!(),
    }
}

#[test]
fn char_mappings() {
    use truetype::char_mapping::{CharMapping, Encoding, Mapping};

    fn filter_out_empty_mappings(mapping: &mut Mapping) {
        match *mapping {
            Mapping::U8(ref mut mapping) => mapping.retain(|_, value| value != &0),
            Mapping::U16(ref mut mapping) => mapping.retain(|_, value| value != &0),
            Mapping::U32(ref mut mapping) => mapping.retain(|_, value| value != &0),
            Mapping::None => {}
        }
    }

    let fixtures = Fixture::all();
    for fixture in fixtures {
        let table = ok!(CharMapping::read(&mut setup(*fixture, Some("cmap"))));
        let expected_mappings = fixture.mappings();
        for (encoding, expected_mapping) in table.encodings.iter().zip(expected_mappings.iter()) {
            match *encoding {
                Encoding::Format14(_) => {}
                _ => {
                    let mut mapping = encoding.mapping();
                    filter_out_empty_mappings(&mut mapping);
                    assert!(mapping == *expected_mapping);
                }
            }
        }
    }
}

#[test]
fn font_header() {
    use truetype::FontHeader;

    let table = ok!(FontHeader::read(&mut setup!(SourceSerif, "head")));
    assert!(format!("{:.3}", f32::from(table.revision)) == "1.017");
    assert!(table.units_per_em == 1000);
    assert!(table.mac_style == 0);
    assert!(table.glyph_mapping_format == 0);
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
    assert!((glyph.min_x, glyph.max_x) == (193, 1034));
    assert!((glyph.min_y, glyph.max_y) == (0, 1462));
    match glyph.description {
        Description::Simple(ref description) => {
            assert!(&description.x == &[193, 841, 0, -841, 104, 633, 0, -633]);
            assert!(&description.y == &[1462, 0, -1462, 0, 104, 0, 1254, 0])
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
            assert!(&offsets[0..10] == &[0, 27, 27, 27, 27, 73, 102, 189, 293, 403]);
        }
        _ => unreachable!(),
    }
}

#[test]
fn horizontal_header() {
    use truetype::HorizontalHeader;

    let table = ok!(HorizontalHeader::read(&mut setup!(SourceSerif, "hhea")));
    assert!(table.ascender == 918);
    assert!(table.descender == -335);
    assert!(table.horizontal_metric_count == 547);
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
    assert!(table.records.len() == 547);
    assert!(table.left_side_bearings.len() == 547 - 547);
    assert!(table.get(42) == (549, 45));
}

#[test]
fn maximum_profile() {
    use truetype::MaximumProfile;

    match ok!(MaximumProfile::read(&mut setup!(SourceSerif, "maxp"))) {
        MaximumProfile::Version0(ref table) => {
            assert!(table.glyph_count == 547);
        }
        _ => unreachable!(),
    }
}

#[test]
fn naming_table() {
    use truetype::NamingTable;

    match ok!(NamingTable::read(&mut setup!(SourceSerif, "name"))) {
        NamingTable::Format0(ref table) => {
            assert!(table.count == 26);
            assert!(ok!(table.strings())[9] == "Frank Grießhammer");
        }
        _ => unreachable!(),
    }
}

#[test]
fn offset_table() {
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
            assert!(table.glyph_count == 938);
            assert!(table.glyph_names.len() == 938);
            assert!(&table.glyph_names[0] == ".notdef");
            assert!(&table.glyph_names[42] == "G");
        }
        _ => unreachable!(),
    }
    match ok!(PostScript::read(&mut setup!(SourceSerif, "post"))) {
        PostScript::Version3(ref table) => {
            assert!(f32::from(table.version) == 3.0);
            assert!(table.underline_position == -75);
        }
        _ => unreachable!(),
    }
}

#[test]
fn windows_metrics() {
    use truetype::WindowsMetrics;

    match ok!(WindowsMetrics::read(&mut setup!(SourceSerif, "OS/2"))) {
        WindowsMetrics::Version3(ref table) => {
            assert!(table.panose == [2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
            assert!(stringify(&table.vendor_id) == "ADBE");
            assert!(table.break_char == 32);
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
