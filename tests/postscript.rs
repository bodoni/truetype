extern crate truetype;

use truetype::Value;

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
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
