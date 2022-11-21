extern crate truetype;

use truetype::Value;

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    use truetype::MaximumProfile;

    match ok!(MaximumProfile::read(&mut setup!(SourceSerif, "maxp"))) {
        MaximumProfile::Version0(ref table) => {
            assert!(table.glyph_count == 547);
        }
        _ => unreachable!(),
    }
}
