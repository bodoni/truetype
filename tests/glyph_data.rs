extern crate truetype;

use truetype::{Value, Walue};

#[macro_use]
mod common;

use common::setup;

#[test]
fn read() {
    use truetype::glyph_data::Description;
    use truetype::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};

    let parameter1 = ok!(FontHeader::read(&mut setup!(OpenSans, "head")));
    let parameter2 = ok!(MaximumProfile::read(&mut setup!(OpenSans, "maxp")));
    let parameter = ok!(GlyphMapping::read(
        &mut setup!(OpenSans, "loca"),
        (&parameter1, &parameter2),
    ));
    let table = ok!(GlyphData::read(&mut setup!(OpenSans, "glyf"), &parameter));
    let glyph = ok!(table[0].as_ref());
    assert!((glyph.min_x, glyph.max_x) == (193, 1034));
    assert!((glyph.min_y, glyph.max_y) == (0, 1462));
    match glyph.description {
        Description::Simple(ref description) => {
            assert!(&description.x == &[193, 841, 0, -841, 104, 633, 0, -633]);
            assert!(&description.y == &[1462, 0, -1462, 0, 104, 0, 1254, 0])
        }
        _ => unreachable!(),
    }
}
