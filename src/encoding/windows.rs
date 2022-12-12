#[inline]
pub fn decode(_: &[u8], encoding_id: u16) -> Option<String> {
    match encoding_id {
        // 0 => Symbol
        // 1 => Unicode BMP
        // 2 => ShiftJIS
        // 3 => PRC
        // 4 => Big5
        // 5 => Wansung
        // 6 => Johab
        // 7 => Reserved
        // 8 => Reserved
        // 9 => Reserved
        // 10 => Unicode full repertoire
        _ => None,
    }
}
