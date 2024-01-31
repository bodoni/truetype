#[macro_use]
mod support;

use truetype::value::Read;

use support::Fixture;

#[test]
fn header() {
    use truetype::tables::CharacterMapping;

    let table = ok!(CharacterMapping::read(&mut setup!(SourceSerif, "cmap")));
    let table = &table.header;
    assert!(table.version == 0);
    assert!(table.table_count == 3);
}

#[test]
fn encoding_format4() {
    use truetype::tables::character_mapping::{CharacterMapping, Encoding};

    let table = ok!(CharacterMapping::read(&mut setup!(SourceSerif, "cmap")));
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
            assert!(table.mapping::<u32>() == Fixture::SourceSerif.mappings()[0]);
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
fn encoding_format6() {
    use truetype::tables::character_mapping::{CharacterMapping, Encoding};

    let table = ok!(CharacterMapping::read(&mut setup!(SourceSerif, "cmap")));
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
fn encoding_formats() {
    use truetype::tables::character_mapping::{CharacterMapping, Encoding};

    let fixtures = &[
        Fixture::MPlus2P,
        Fixture::OpenSans,
        Fixture::SourceSerif,
        Fixture::VeraMono,
    ];
    for fixture in fixtures {
        let table = ok!(CharacterMapping::read(&mut support::setup(
            *fixture,
            Some("cmap"),
        )));
        let expected_mappings = fixture.mappings();
        assert!(table.encodings.len() == expected_mappings.len());
        for (encoding, expected_mapping) in table.encodings.iter().zip(expected_mappings) {
            let mut mapping = match encoding {
                &Encoding::Format0(ref encoding) => encoding.mapping::<u32>(),
                &Encoding::Format4(ref encoding) => encoding.mapping::<u32>(),
                &Encoding::Format6(ref encoding) => encoding.mapping::<u32>(),
                &Encoding::Format12(ref encoding) => encoding.mapping::<u32>(),
                &Encoding::Format14(_) => continue,
                _ => unreachable!(),
            };
            mapping.retain(|_, value| value != &0);
            assert!(mapping == expected_mapping);
        }
    }
}

#[test]
fn records() {
    use truetype::tables::CharacterMapping;

    let table = ok!(CharacterMapping::read(&mut setup!(SourceSerif, "cmap")));
    let tables = &table.records;
    assert!(tables.len() == 3);
    assert!(tables[0].platform_id == 0);
    assert!(tables[0].encoding_id == 3);
    assert!(tables[1].platform_id == 1);
    assert!(tables[1].encoding_id == 0);
    assert!(tables[2].platform_id == 3);
    assert!(tables[2].encoding_id == 1);
}
