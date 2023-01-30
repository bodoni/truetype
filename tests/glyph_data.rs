extern crate truetype;

#[macro_use]
mod support;

mod open_sans {
    use truetype::{Value, Walue};

    use crate::support::setup;

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
        assert_eq!((glyph.min_x, glyph.max_x), (193, 1034));
        assert_eq!((glyph.min_y, glyph.max_y), (0, 1462));
        match glyph.description {
            Description::Simple(ref description) => {
                assert_eq!(&description.x, &[193, 841, 0, -841, 104, 633, 0, -633]);
                assert_eq!(&description.y, &[1462, 0, -1462, 0, 104, 0, 1254, 0])
            }
            _ => unreachable!(),
        }
    }
}

mod ubuntu_condensed {
    use truetype::{Value, Walue};

    use crate::support::setup;

    #[test]
    #[cfg_attr(not(feature = "ignore-invalid-composite-glyph-flags"), should_panic)]
    fn read() {
        use truetype::glyph_data::Description;
        use truetype::{FontHeader, GlyphData, GlyphMapping, MaximumProfile};

        let parameter1 = ok!(FontHeader::read(&mut setup!(UbuntuCondensed, "head")));
        let parameter2 = ok!(MaximumProfile::read(&mut setup!(UbuntuCondensed, "maxp")));
        let parameter = ok!(GlyphMapping::read(
            &mut setup!(UbuntuCondensed, "loca"),
            (&parameter1, &parameter2),
        ));
        let table = ok!(GlyphData::read(
            &mut setup!(UbuntuCondensed, "glyf"),
            &parameter,
        ));
        let glyph = ok!(table[0].as_ref());
        assert_eq!((glyph.min_x, glyph.max_x), (50, 450));
        assert_eq!((glyph.min_y, glyph.max_y), (0, 750));
        match glyph.description {
            Description::Simple(ref description) => {
                assert_eq!(&description.x, &[50, 0, 400, 0, -50, 0, -300, 0]);
                assert_eq!(&description.y, &[0, 750, 0, -750, 50, 650, 0, -650])
            }
            _ => unreachable!(),
        }
    }
}

mod zen_loop {
    use truetype::{Value, Walue};

    use crate::support::setup;

    #[test]
    fn read() {
        use truetype::glyph_data::Description;
        use truetype::glyph_data::{Arguments, GlyphData, Options};
        use truetype::{FontHeader, GlyphMapping, MaximumProfile};

        let parameter1 = ok!(FontHeader::read(&mut setup!(ZenLoop, "head")));
        let parameter2 = ok!(MaximumProfile::read(&mut setup!(ZenLoop, "maxp")));
        let parameter = ok!(GlyphMapping::read(
            &mut setup!(ZenLoop, "loca"),
            (&parameter1, &parameter2),
        ));
        let table = ok!(GlyphData::read(&mut setup!(ZenLoop, "glyf"), &parameter,));
        let glyph = ok!(table[72].as_ref());
        match glyph.description {
            Description::Composite(ref description) => {
                assert_eq!(description.components.len(), 1);
                assert_eq!(description.components[0].glyph_id, 70);
                match description.components[0].arguments {
                    Arguments::Offsets(x, y) => {
                        assert_eq!(x, 298);
                        assert_eq!(y, 0);
                    }
                    _ => unreachable!(),
                }
                match description.components[0].options {
                    Options::Vector(x, y) => {
                        assert_eq!(-1.0f32, x.into());
                        assert_eq!(1.0f32, y.into());
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
