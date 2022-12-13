extern crate truetype;

#[macro_use]
mod common;

mod open_sans {
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
        use truetype::NamingTable;

        match ok!(NamingTable::read(&mut setup!(OpenSans, "name"))) {
            NamingTable::Format0(ref table) => {
                assert_eq!(
                    table.strings().unwrap(),
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
            _ => unreachable!(),
        }
    }
}

mod source_serif {
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
        use truetype::NamingTable;

        match ok!(NamingTable::read(&mut setup!(SourceSerif, "name"))) {
            NamingTable::Format0(ref table) => {
                assert_eq!(table.count, 26);
                assert_eq!(ok!(table.strings())[9], "Frank Grießhammer");
            }
            _ => unreachable!(),
        }
    }
}
