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
        // 1 => Japanese,
        // 2 => Chinese (Traditional),
        // 3 => Korean,
        // 4 => Arabic,
        // 5 => Hebrew,
        // 6 => Greek,
        // 7 => Russian,
        // 8 => RSymbol,
        // 9 => Devanagari,
        // 10 => Gurmukhi,
        // 11 => Gujarati,
        // 12 => Oriya,
        // 13 => Bengali,
        // 14 => Tamil,
        // 15 => Telugu,
        // 16 => Kannada,
        // 17 => Malayalam,
        // 18 => Sinhalese,
        // 19 => Burmese,
        // 20 => Khmer,
        // 21 => Thai,
        // 22 => Laotian,
        // 23 => Georgian,
        // 24 => Armenian,
        // 25 => Chinese (Simplified),
        // 26 => Tibetan,
        // 27 => Mongolian,
        // 28 => Geez,
        // 29 => Slavic,
        // 30 => Vietnamese,
        // 31 => Sindhi,
        // 32 => Uninterpreted,
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
