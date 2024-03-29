#[macro_use]
mod support;

use truetype::value::Read;

#[test]
fn read() {
    use truetype::tables::MaximumProfile;

    match ok!(MaximumProfile::read(&mut setup!(SourceSerif, "maxp"))) {
        MaximumProfile::Version0(ref table) => {
            assert!(table.glyph_count == 547);
        }
        _ => unreachable!(),
    }
}
