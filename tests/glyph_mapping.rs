extern crate truetype;

#[macro_use]
mod support;

use truetype::{Value, Walue};

use support::setup;

#[test]
fn read() {
    use truetype::{FontHeader, GlyphMapping, MaximumProfile};

    let parameter1 = ok!(FontHeader::read(&mut setup!(OpenSans, "head")));
    let parameter2 = ok!(MaximumProfile::read(&mut setup!(OpenSans, "maxp")));
    match ok!(GlyphMapping::read(
        &mut setup!(OpenSans, "loca"),
        (&parameter1, &parameter2),
    )) {
        GlyphMapping::HalfOffsets(ref offsets) => {
            assert!(&offsets[0..10] == &[0, 27, 27, 27, 27, 73, 102, 189, 293, 403]);
        }
        _ => unreachable!(),
    }
}
