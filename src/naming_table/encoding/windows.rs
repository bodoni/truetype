use crate::naming_table::encoding::unicode;
use crate::naming_table::encoding::EncodingID;

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
