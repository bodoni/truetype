use crate::tables::names::encoding::EncodingID;
use crate::Result;

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
    if data.len() % 2 != 0 {
        return None
    }
    let data = data
        .chunks_exact(2)
        .map(TryInto::try_into)
        .map(std::result::Result::ok)
        .map(Option::unwrap)
        .map(u16::from_be_bytes)
        .collect::<Vec<_>>();
    String::from_utf16(&data).ok()
}

pub fn encode(_: &str, _: EncodingID, _: &mut Vec<u8>) -> Result<()> {
    Ok(())
}

pub fn encode_utf16(value: &str, data: &mut Vec<u8>) {
    data.extend(
        value
            .encode_utf16()
            .map(u16::to_be_bytes)
            .flat_map(<[u8; 2]>::into_iter),
    );
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

    #[test]
    fn encode_utf16() {
        let mut data = vec![];
        super::encode_utf16("\u{24B62}", &mut data);
        assert_eq!(data, &[0xD8, 0x52, 0xDF, 0x62]);
    }
}
