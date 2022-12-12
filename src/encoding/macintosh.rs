#[rustfmt::skip]
const ROMAN: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
    'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
    'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
    'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥',
    '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', '¿',
    '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
    'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄',
    '€', '‹', '›', 'ﬁ', 'ﬂ', '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á',
    'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
    'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ',
];

// Reference:
// https://github.com/opentypejs/opentype.js/blob/c37fcdfbd89c1bd0aac1cecb2b287dfb7d00cee0/src/types.js#L463-L482
pub fn decode(bytes: &[u8], encoding_id: u16) -> Option<String> {
    let table = match encoding_id {
        0 => &ROMAN,
        // 1 => &JAPANESE,
        // 2 => &CHINESE_TRADITIONAL,
        // 3 => &KOREAN,
        // 4 => &ARABIC,
        // 5 => &HEBREW,
        // 6 => &GREEK,
        // 7 => &RUSSIAN,
        // 8 => &RSYMBOL,
        // 9 => &DEVANAGARI,
        // 10 => &GURMUKHI,
        // 11 => &GUJARATI,
        // 12 => &ORIYA,
        // 13 => &BENGALI,
        // 14 => &TAMIL,
        // 15 => &TELUGU,
        // 16 => &KANNADA,
        // 17 => &MALAYALAM,
        // 18 => &SINHALESE,
        // 19 => &BURMESE,
        // 20 => &KHMER,
        // 21 => &THAI,
        // 22 => &LAOTIAN,
        // 23 => &GEORGIAN,
        // 24 => &ARMENIAN,
        // 25 => &CHINESE_SIMPLIFIED,
        // 26 => &TIBETAN,
        // 27 => &MONGOLIAN,
        // 28 => &GEEZ,
        // 29 => &SLAVIC,
        // 30 => &VIETNAMESE,
        // 31 => &SINDHI,
        // 32 => &UNINTERPRETED,
        _ => return None,
    };

    let mut string = String::new();
    for &byte in bytes {
        if byte <= 0x7F {
            string.push(byte as char);
        } else {
            string.push(table[(byte & 0x7F) as usize]);
        }
    }

    Some(string)
}
