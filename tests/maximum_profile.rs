extern crate truetype;

#[macro_use]
mod support;

use truetype::Value;

use support::setup;

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
