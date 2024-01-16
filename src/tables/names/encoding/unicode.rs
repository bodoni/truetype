use crate::tables::names::encoding::EncodingID;

#[inline]
pub fn decode(data: &[u8], encoding_id: EncodingID) -> Option<String> {
    match encoding_id {
        // 0 => Unicode 1.0 semantics—deprecated
        // 1 => Unicode 1.1 semantics—deprecated
        // 2 => ISO/IEC 10646 semantics—deprecated
        3 => decode_utf16(data), // Unicode 2.0 and onwards semantics, Unicode BMP only
        4 => decode_utf16(data), // Unicode 2.0 and onwards semantics, Unicode full repertoire
        _ => None,
    }
}

pub fn decode_utf16(data: &[u8]) -> Option<String> {
    use std::io::Cursor;
    use typeface::Tape;

    let mut tape = Cursor::new(data);
    match tape.take_given::<Vec<_>>(data.len() / 2) {
        Ok(data) => match String::from_utf16(&data) {
            Ok(string) => Some(string),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_utf16() {
        assert_eq!(
            super::decode_utf16(&[0xD8, 0x52, 0xDF, 0x62]).unwrap(),
            "\u{24B62}",
        );
    }
}
