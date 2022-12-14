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
        let names = table.get_all();
        let ids: Vec<_> = names.iter().map(|(id, _)| *id).collect();
        let strings: Vec<_> = names.into_iter().map(|(_, string)| ok!(string)).collect();
        #[rustfmt::skip]
        assert_eq!(
            ids,
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14,
                0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14,
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
        use truetype::naming_table::{NamingTable, PredefinedName};

        let table = ok!(NamingTable::read(&mut setup!(SourceSerif, "name")));
        assert_eq!(
            ok!(table.get(PredefinedName::UniqueFontID)),
            "1.017;ADBE;SourceSerifPro-Regular;ADOBE",
        );
        assert_eq!(
            ok!(table.get(PredefinedName::FontFamilyName)),
            "Source Serif Pro",
        );
        assert_eq!(
            ok!(table.get(PredefinedName::DesignerName)),
            "Frank Grießhammer",
        );
        assert!(table
            .get(PredefinedName::PostScriptCIDFindFontName)
            .is_none());
    }
}
