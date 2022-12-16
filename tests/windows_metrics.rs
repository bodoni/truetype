extern crate truetype;

#[macro_use]
mod common;

use truetype::{Tag, Value};

use common::setup;

#[test]
fn read() {
    use truetype::WindowsMetrics;

    match ok!(WindowsMetrics::read(&mut setup!(SourceSerif, "OS/2"))) {
        WindowsMetrics::Version3(ref table) => {
            assert_eq!(table.panose, [2, 4, 6, 3, 5, 4, 5, 2, 2, 4]);
            assert_eq!(table.vendor_id, Tag(*b"ADBE"));
            assert_eq!(table.break_char, 32);
        }
        _ => unreachable!(),
    }
}
