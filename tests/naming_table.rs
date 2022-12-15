extern crate truetype;

#[macro_use]
mod common;

mod open_sans {
    use truetype::naming_table::{NameID, NamingTable};
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
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
                NameID::CopyrightNotice,
                NameID::FontFamilyName,
                NameID::FontSubfamilyName,
                NameID::UniqueFontID,
                NameID::FullFontName,
                NameID::VersionString,
                NameID::PostScriptFontName,
                NameID::Trademark,
                NameID::ManufacturerName,
                NameID::VendorURL,
                NameID::DesignerURL,
                NameID::LicenseDescription,
                NameID::LicenseURL,

                NameID::CopyrightNotice,
                NameID::FontFamilyName,
                NameID::FontSubfamilyName,
                NameID::UniqueFontID,
                NameID::FullFontName,
                NameID::VersionString,
                NameID::PostScriptFontName,
                NameID::Trademark,
                NameID::ManufacturerName,
                NameID::VendorURL,
                NameID::DesignerURL,
                NameID::LicenseDescription,
                NameID::LicenseURL,
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
    use std::collections::HashMap;

    use truetype::naming_table::{NameID, NamingTable};
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
        let table = ok!(NamingTable::read(&mut setup!(SourceSerif, "name")));
        let names: HashMap<_, _> = table
            .decode()
            .into_iter()
            .filter(|(_, language_tag, _)| ok!(language_tag.as_deref()) == "en")
            .map(|(name_id, _, string)| (name_id, ok!(string)))
            .collect();
        assert_eq!(
            names[&NameID::UniqueFontID],
            "1.017;ADBE;SourceSerifPro-Regular;ADOBE",
        );
        assert_eq!(names[&NameID::FontFamilyName], "Source Serif Pro");
        assert_eq!(names[&NameID::DesignerName], "Frank Grießhammer");
        assert!(!names.contains_key(&NameID::PostScriptCIDFindFontName));
    }
}
