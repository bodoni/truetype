#[macro_use]
mod support;

mod css_test {
    use truetype::tables::names::Names;
    use truetype::value::Read;

    #[cfg_attr(not(feature = "ignore-invalid-language-ids"), should_panic)]
    #[test]
    fn read() {
        let table = ok!(Names::read(&mut setup!(CSSTest, "name")));
        let _: Vec<_> = table.iter().collect();
    }
}

mod open_sans {
    use std::fs::File;
    use std::io::Cursor;

    use truetype::tables::names::{NameID, Names};
    use truetype::tape::Read as TapeRead;
    use truetype::tape::Write;
    use truetype::value::Read as ValueRead;

    #[test]
    fn read() {
        let table = ok!(Names::read(&mut setup!(OpenSans, "name")));
        test(&table);
    }

    #[test]
    fn write() {
        let path = crate::support::Fixture::OpenSans.path();
        let offset = crate::support::Fixture::OpenSans.offset("name");
        let mut file = ok!(File::open(path));

        ok!(file.jump(offset));
        let one = ok!(Names::read(&mut file));
        let size = (ok!(file.position()) - offset) as usize;

        let records = one.iter().map(|(ids, value)| (ids, ok!(value)));
        let language_tags = one.language_tags().map(Option::unwrap);
        let other = ok!(Names::from_iter(
            records,
            language_tags,
            &mut Default::default(),
        ));
        test(&other);
        match (&one, &other) {
            (Names::Format0(one), Names::Format1(other)) => {
                assert_eq!(one.data.len(), other.data.len() - 48);
            }
            _ => unreachable!(),
        }

        let mut cursor = Cursor::new(vec![]);
        ok!(cursor.give(&other));
        assert_eq!(size, cursor.into_inner().len() - 48 - 2);
    }

    fn test(table: &Names) {
        let records = table.iter().collect::<Vec<_>>();
        let name_ids = records
            .iter()
            .map(|((_, _, _, name_id), _)| *name_id)
            .collect::<Vec<_>>();
        let language_tags = table.language_tags().collect::<Vec<_>>();
        let language_tags = records
            .iter()
            .map(|((_, _, language_id, _), _)| ok!(language_id.tag(&language_tags)))
            .collect::<Vec<_>>();
        let values = records
            .iter()
            .map(|(_, value)| ok!(value.as_deref()))
            .collect::<Vec<_>>();
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
            values,
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

    use truetype::tables::names::{NameID, Names};
    use truetype::value::Read;

    #[test]
    fn read() {
        let table = ok!(Names::read(&mut setup!(SourceSerif, "name")));
        let language_tags = table.language_tags().collect::<Vec<_>>();
        let records = table
            .iter()
            .rev()
            .filter(|((_, _, language_id, _), value)| {
                value.is_some()
                    && language_id
                        .tag(&language_tags)
                        .map_or(false, |tag| tag.starts_with("en"))
            })
            .map(|((_, _, _, name_id), value)| (name_id, ok!(value)))
            .collect::<HashMap<_, _>>();
        assert_eq!(
            records[&NameID::UniqueFontID],
            "1.017;ADBE;SourceSerifPro-Regular;ADOBE",
        );
        assert_eq!(records[&NameID::FontFamilyName], "Source Serif Pro");
        assert_eq!(records[&NameID::DesignerName], "Frank Grießhammer");
        assert!(!records.contains_key(&NameID::PostScriptCIDFindFontName));
    }
}
