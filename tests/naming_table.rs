extern crate truetype;

#[macro_use]
mod common;

mod open_sans {
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
        use truetype::NamingTable;

        let table = ok!(NamingTable::read(&mut setup!(OpenSans, "name")));
        let names = table.decode();
        let name_ids: Vec<_> = names.iter().map(|(name_id, _, _)| *name_id).collect();
        let language_tags: Vec<_> = names
            .iter()
            .map(|(_, language_tag, _)| ok!(language_tag.as_deref()))
            .collect();
        let strings: Vec<_> = names
            .iter()
            .map(|(_, _, string)| ok!(string.as_deref()))
            .collect();
        #[rustfmt::skip]
        assert_eq!(
            name_ids,
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14,
                0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14,
            ],
        );
        #[rustfmt::skip]
        assert_eq!(
            language_tags,
            &[
                "en", "en", "en", "en", "en", "en", "en", "en", "en", "en", "en", "en", "en",
                "en", "en", "en", "en", "en", "en", "en", "en", "en", "en", "en", "en", "en",
            ],
        );
        assert_eq!(
            strings,
            &[
                "Digitized data copyright © 2010-2011, Google Corporation.",
                "Open Sans",
                "Italic",
                "Ascender - Open Sans Italic Build 100",
                "Open Sans Italic",
                "Version 1.10",
                "OpenSans-Italic",
                "Open Sans is a trademark of Google and may be registered in certain jurisdictions.",
                "Ascender Corporation",
                "http://www.ascendercorp.com/",
                "http://www.ascendercorp.com/typedesigners.html",
                "Licensed under the Apache License, Version 2.0",
                "http://www.apache.org/licenses/LICENSE-2.0",

                "Digitized data copyright © 2010-2011, Google Corporation.",
                "Open Sans",
                "Italic",
                "Ascender - Open Sans Italic Build 100",
                "Open Sans Italic",
                "Version 1.10",
                "OpenSans-Italic",
                "Open Sans is a trademark of Google and may be registered in certain jurisdictions.",
                "Ascender Corporation",
                "http://www.ascendercorp.com/",
                "http://www.ascendercorp.com/typedesigners.html",
                "Licensed under the Apache License, Version 2.0",
                "http://www.apache.org/licenses/LICENSE-2.0",
            ],
        );
    }
}

mod source_serif {
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
        use truetype::naming_table::{Name, NamingTable};

        let table = ok!(NamingTable::read(&mut setup!(SourceSerif, "name")));
        let names = table.decode();
        assert_eq!(
            names
                .iter()
                .find(|(name_id, _, _)| *name_id == Name::UniqueFontID.into())
                .map(|(name_id, language_tag, string)| (
                    ok!(Name::try_from(*name_id)),
                    ok!(language_tag.as_deref()),
                    ok!(string.as_deref())
                ))
                .unwrap(),
            (
                Name::UniqueFontID,
                "en",
                "1.017;ADBE;SourceSerifPro-Regular;ADOBE"
            ),
        );
        assert_eq!(
            names
                .iter()
                .find(|(name_id, _, _)| *name_id == Name::FontFamilyName.into())
                .map(|(name_id, language_tag, string)| (
                    ok!(Name::try_from(*name_id)),
                    ok!(language_tag.as_deref()),
                    ok!(string.as_deref())
                ))
                .unwrap(),
            (Name::FontFamilyName, "en", "Source Serif Pro"),
        );
        assert_eq!(
            names
                .iter()
                .find(|(name_id, _, _)| *name_id == Name::DesignerName.into())
                .map(|(name_id, language_tag, string)| (
                    ok!(Name::try_from(*name_id)),
                    ok!(language_tag.as_deref()),
                    ok!(string.as_deref())
                ))
                .unwrap(),
            (Name::DesignerName, "en", "Frank Grießhammer"),
        );
        assert!(names
            .iter()
            .find(|(name_id, _, _)| *name_id == Name::PostScriptCIDFindFontName.into())
            .is_none());
    }
}
