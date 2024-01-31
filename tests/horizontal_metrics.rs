#[macro_use]
mod support;

use truetype::value::Read as ValueRead;
use truetype::walue::Read as WalueRead;

#[test]
fn read() {
    use truetype::tables::{HorizontalHeader, HorizontalMetrics, MaximumProfile};

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
