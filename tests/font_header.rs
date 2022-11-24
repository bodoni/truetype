extern crate truetype;

use truetype::{FontHeader, Value};

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    let table = ok!(FontHeader::read(&mut setup!(SourceSerif, "head")));
    assert_eq!(format!("{:.3}", f32::from(table.revision)), "1.017");
    assert_eq!(table.units_per_em, 1000);
    assert_eq!(table.mac_style, 0);
    assert_eq!(table.glyph_mapping_format, 0);
}
