#[inline]
pub fn decode(_: &[u8], encoding_id: u16) -> Option<String> {
    match encoding_id {
        // 0 => Unicode 1.0 semantics—deprecated
        // 1 => Unicode 1.1 semantics—deprecated
        // 2 => ISO/IEC 10646 semantics—deprecated
        // 3 => Unicode 2.0 and onwards semantics, Unicode BMP only
        // 4 => Unicode 2.0 and onwards semantics, Unicode full repertoire
        _ => None,
    }
}
