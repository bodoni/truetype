extern crate truetype;

#[macro_use]
mod common;

mod kaushan_script {
    use truetype::Value;

    use crate::common::setup;

    #[should_panic]
    #[test]
    fn read() {
        use truetype::OffsetTable;

        let mut file = setup!(KaushanScript);
        let OffsetTable { header, records } = ok!(OffsetTable::read(&mut file));
        assert_eq!(header.table_count, 18);
        assert_eq!(records.len(), 18);
        let (_, failures): (Vec<_>, Vec<_>) = records
            .iter()
            .map(|record| (record.tag, record.checksum, ok!(record.checksum(&mut file))))
            .partition(|(_, left, right)| left == right);
        assert_eq!(failures.len(), 0, "{:?}", failures);
    }
}

mod source_serif {
    use truetype::Value;

    use crate::common::setup;

    #[test]
    fn read() {
        use truetype::OffsetTable;

        let mut file = setup!(SourceSerif);
        let OffsetTable { header, records } = ok!(OffsetTable::read(&mut file));
        assert_eq!(header.table_count, 12);
        assert_eq!(header.search_range, 8 * 16);
        assert_eq!(header.entry_selector, 3);
        assert_eq!(
            header.range_shift,
            header.table_count * 16 - header.search_range
        );
        assert_eq!(records.len(), 12);
        let (_, failures): (Vec<_>, Vec<_>) = records
            .iter()
            .map(|record| (record.tag, record.checksum, ok!(record.checksum(&mut file))))
            .partition(|(_, left, right)| left == right);
        assert_eq!(failures.len(), 0, "{:?}", failures);
    }
}
