extern crate truetype;

#[macro_use]
mod common;

use truetype::Value;

use common::setup;

#[test]
fn read() {
    use truetype::HorizontalHeader;

    let table = ok!(HorizontalHeader::read(&mut setup!(SourceSerif, "hhea")));
    assert!(table.ascender == 918);
    assert!(table.descender == -335);
    assert!(table.horizontal_metric_count == 547);
}
