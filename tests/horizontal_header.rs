#[macro_use]
mod support;

use truetype::value::Read;

#[test]
fn read() {
    use truetype::tables::HorizontalHeader;

    let table = ok!(HorizontalHeader::read(&mut setup!(SourceSerif, "hhea")));
    assert!(table.ascender == 918);
    assert!(table.descender == -335);
    assert!(table.horizontal_metric_count == 547);
}
