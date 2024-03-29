use crate::tables::names::encoding::unicode;
use crate::tables::names::encoding::EncodingID;
use crate::Result;

#[inline]
pub fn decode(data: &[u8], encoding_id: EncodingID) -> Option<String> {
    match encoding_id {
        // 0 => Symbol
        1 => unicode::decode_utf16(data), // Unicode BMP
        // 2 => ShiftJIS
        // 3 => PRC
        // 4 => Big5
        // 5 => Wansung
        // 6 => Johab
        // 7 => Reserved
        // 8 => Reserved
        // 9 => Reserved
        10 => unicode::decode_utf16(data), // Unicode full repertoire
        _ => None,
    }
}

pub fn encode(value: &str, encoding_id: EncodingID, data: &mut Vec<u8>) -> Result<()> {
    match encoding_id {
        // 0 => Symbol
        1 => unicode::encode_utf16(value, data), // Unicode BMP
        // 2 => ShiftJIS
        // 3 => PRC
        // 4 => Big5
        // 5 => Wansung
        // 6 => Johab
        // 7 => Reserved
        // 8 => Reserved
        // 9 => Reserved
        10 => unicode::encode_utf16(value, data), // Unicode full repertoire
        _ => raise!("found an unknown Windows encoding ({encoding_id})"),
    }
    Ok(())
}
