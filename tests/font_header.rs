extern crate truetype;

use truetype::Value;

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    use truetype::FontHeader;

    let table = ok!(FontHeader::read(&mut setup!(SourceSerif, "head")));
    assert!(format!("{:.3}", f32::from(table.revision)) == "1.017");
    assert!(table.units_per_em == 1000);
    assert!(table.mac_style == 0);
    assert!(table.glyph_mapping_format == 0);
}
