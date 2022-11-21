extern crate truetype;

#[macro_use]
mod common;

mod kaushan_script {
    use truetype::{FontHeader, Value};

    use crate::common::setup;

    #[test]
    fn read() {
        let table = ok!(FontHeader::read(&mut setup!(KaushanScript, "head")));
        assert_eq!(format!("{:.3}", f32::from(table.revision)), "1.002");
        assert_eq!(table.units_per_em, 1000);
        assert_eq!(table.min_x, -208);
        assert_eq!(table.min_y, -367);
        assert_eq!(table.max_x, 1156);
        assert_eq!(table.max_y, 1084);
        assert_eq!(table.mac_style, 0);
        assert_eq!(table.lowest_ppem, 9);
        assert_eq!(table.direction_hint, 2);
        assert_eq!(table.glyph_mapping_format, 1);
        assert_eq!(table.glyph_data_format, 0);
    }
}

mod source_serif {
    use truetype::{FontHeader, Value};

    use crate::common::setup;

    #[test]
    fn read() {
        let table = ok!(FontHeader::read(&mut setup!(SourceSerif, "head")));
        assert_eq!(format!("{:.3}", f32::from(table.revision)), "1.017");
        assert_eq!(table.units_per_em, 1000);
        assert_eq!(table.mac_style, 0);
        assert_eq!(table.glyph_mapping_format, 0);
    }
}
