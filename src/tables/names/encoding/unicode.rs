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
    let data = data
        .chunks(2)
        .map(TryInto::try_into)
        .map(Result::ok)
        .map(Option::unwrap)
        .map(u16::from_be_bytes)
        .collect::<Vec<_>>();
    String::from_utf16(&data).ok()
}

#[cfg(test)]
mod tests {
    macro_rules! ok(($result:expr) => ($result.unwrap()));

    #[test]
    fn decode_utf16() {
        assert_eq!(
            ok!(super::decode_utf16(&[0xD8, 0x52, 0xDF, 0x62])),
            "\u{24B62}",
        );
    }
}
