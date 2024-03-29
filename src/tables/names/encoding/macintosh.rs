// Reference:
// https://github.com/opentypejs/opentype.js/blob/c37fcdfbd89c1bd0aac1cecb2b287dfb7d00cee0/src/types.js#L463-L482

use std::collections::HashMap;

use crate::tables::names::encoding::EncodingID;
use crate::tables::names::language::{LanguageID, Macintosh};
use crate::Result;

#[derive(Default)]
pub struct Context {
    mapping: HashMap<(u16, u16), HashMap<char, u8>>,
}

#[rustfmt::skip]
const MACINTOSH: [char; 128] = [
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

#[rustfmt::skip]
const X_MAC_CE: [char; 128] = [
    'Ä', 'Ā', 'ā', 'É', 'Ą', 'Ö', 'Ü', 'á', 'ą', 'Č', 'ä', 'č', 'Ć',
    'ć', 'é', 'Ź', 'ź', 'Ď', 'í', 'ď', 'Ē', 'ē', 'Ė', 'ó', 'ė', 'ô',
    'ö', 'õ', 'ú', 'Ě', 'ě', 'ü', '†', '°', 'Ę', '£', '§', '•', '¶',
    'ß', '®', '©', '™', 'ę', '¨', '≠', 'ģ', 'Į', 'į', 'Ī', '≤', '≥',
    'ī', 'Ķ', '∂', '∑', 'ł', 'Ļ', 'ļ', 'Ľ', 'ľ', 'Ĺ', 'ĺ', 'Ņ', 'ņ',
    'Ń', '¬', '√', 'ń', 'Ň', '∆', '«', '»', '…', ' ', 'ň', 'Ő', 'Õ',
    'ő', 'Ō', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ō', 'Ŕ', 'ŕ',
    'Ř', '‹', '›', 'ř', 'Ŗ', 'ŗ', 'Š', '‚', '„', 'š', 'Ś', 'ś', 'Á',
    'Ť', 'ť', 'Í', 'Ž', 'ž', 'Ū', 'Ó', 'Ô', 'ū', 'Ů', 'Ú', 'ů', 'Ű',
    'ű', 'Ų', 'ų', 'Ý', 'ý', 'ķ', 'Ż', 'Ł', 'ż', 'Ģ', 'ˇ',
];

#[rustfmt::skip]
const X_MAC_CROATIAN: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
    'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
    'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
    'ß', '®', 'Š', '™', '´', '¨', '≠', 'Ž', 'Ø', '∞', '±', '≤', '≥',
    '∆', 'µ', '∂', '∑', '∏', 'š', '∫', 'ª', 'º', 'Ω', 'ž', 'ø', '¿',
    '¡', '¬', '√', 'ƒ', '≈', 'Ć', '«', 'Č', '…', ' ', 'À', 'Ã', 'Õ',
    'Œ', 'œ', 'Đ', '—', '“', '”', '‘', '’', '÷', '◊', '', '©', '⁄',
    '€', '‹', '›', 'Æ', '»', '–', '·', '‚', '„', '‰', 'Â', 'ć', 'Á',
    'č', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', 'đ', 'Ò', 'Ú', 'Û', 'Ù',
    'ı', 'ˆ', '˜', '¯', 'π', 'Ë', '˚', '¸', 'Ê', 'æ', 'ˇ',
];

#[rustfmt::skip]
const X_MAC_CYRILLIC: [char; 128] = [
    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М',
    'Н', 'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ',
    'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я', '†', '°', 'Ґ', '£', '§', '•', '¶',
    'І', '®', '©', '™', 'Ђ', 'ђ', '≠', 'Ѓ', 'ѓ', '∞', '±', '≤', '≥',
    'і', 'µ', 'ґ', 'Ј', 'Є', 'є', 'Ї', 'ї', 'Љ', 'љ', 'Њ', 'њ', 'ј',
    'Ѕ', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'Ћ', 'ћ', 'Ќ',
    'ќ', 'ѕ', '–', '—', '“', '”', '‘', '’', '÷', '„', 'Ў', 'ў', 'Џ',
    'џ', '№', 'Ё', 'ё', 'я', 'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з',
    'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф',
    'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', '\u{FFFD}',
];

#[rustfmt::skip]
const X_MAC_GAELIC: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
    'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
    'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
    'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', 'Ḃ', '±', '≤', '≥',
    'ḃ', 'Ċ', 'ċ', 'Ḋ', 'ḋ', 'Ḟ', 'ḟ', 'Ġ', 'ġ', 'Ṁ', 'æ', 'ø', 'ṁ',
    'Ṗ', 'ṗ', 'ɼ', 'ƒ', 'ſ', 'Ṡ', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
    'Œ', 'œ', '–', '—', '“', '”', '‘', '’', 'ṡ', 'ẛ', 'ÿ', 'Ÿ', 'Ṫ',
    '€', '‹', '›', 'Ŷ', 'ŷ', 'ṫ', '·', 'Ỳ', 'ỳ', '⁊', 'Â', 'Ê', 'Á',
    'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '♣', 'Ò', 'Ú', 'Û', 'Ù',
    'ı', 'Ý', 'ý', 'Ŵ', 'ŵ', 'Ẅ', 'ẅ', 'Ẁ', 'ẁ', 'Ẃ', 'ẃ',
];

#[rustfmt::skip]
const X_MAC_GREEK: [char; 128] = [
    'Ä', '¹', '²', 'É', '³', 'Ö', 'Ü', '΅', 'à', 'â', 'ä', '΄', '¨',
    'ç', 'é', 'è', 'ê', 'ë', '£', '™', 'î', 'ï', '•', '½', '‰', 'ô',
    'ö', '¦', '€', 'ù', 'û', 'ü', '†', 'Γ', 'Δ', 'Θ', 'Λ', 'Ξ', 'Π',
    'ß', '®', '©', 'Σ', 'Ϊ', '§', '≠', '°', '·', 'Α', '±', '≤', '≥',
    '¥', 'Β', 'Ε', 'Ζ', 'Η', 'Ι', 'Κ', 'Μ', 'Φ', 'Ϋ', 'Ψ', 'Ω', 'ά',
    'Ν', '¬', 'Ο', 'Ρ', '≈', 'Τ', '«', '»', '…', ' ', 'Υ', 'Χ', 'Ά',
    'Έ', 'œ', '–', '―', '“', '”', '‘', '’', '÷', 'Ή', 'Ί', 'Ό', 'Ύ',
    'έ', 'ή', 'ί', 'ό', 'Ώ', 'ύ', 'α', 'β', 'ψ', 'δ', 'ε', 'φ', 'γ',
    'η', 'ι', 'ξ', 'κ', 'λ', 'μ', 'ν', 'ο', 'π', 'ώ', 'ρ', 'σ', 'τ',
    'θ', 'ω', 'ς', 'χ', 'υ', 'ζ', 'ϊ', 'ϋ', 'ΐ', 'ΰ', '\u{00AD}',
];

#[rustfmt::skip]
const X_MAC_ICELANDIC: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
    'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
    'ö', 'õ', 'ú', 'ù', 'û', 'ü', 'Ý', '°', '¢', '£', '§', '•', '¶',
    'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥',
    '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', '¿',
    '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
    'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄',
    '€', 'Ð', 'ð', 'Þ', 'þ', 'ý', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á',
    'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
    'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ',
];

#[rustfmt::skip]
const X_MAC_INUIT: [char; 128] = [
    'ᐃ', 'ᐄ', 'ᐅ', 'ᐆ', 'ᐊ', 'ᐋ', 'ᐱ', 'ᐲ', 'ᐳ', 'ᐴ', 'ᐸ', 'ᐹ', 'ᑉ',
    'ᑎ', 'ᑏ', 'ᑐ', 'ᑑ', 'ᑕ', 'ᑖ', 'ᑦ', 'ᑭ', 'ᑮ', 'ᑯ', 'ᑰ', 'ᑲ', 'ᑳ',
    'ᒃ', 'ᒋ', 'ᒌ', 'ᒍ', 'ᒎ', 'ᒐ', 'ᒑ', '°', 'ᒡ', 'ᒥ', 'ᒦ', '•', '¶',
    'ᒧ', '®', '©', '™', 'ᒨ', 'ᒪ', 'ᒫ', 'ᒻ', 'ᓂ', 'ᓃ', 'ᓄ', 'ᓅ', 'ᓇ',
    'ᓈ', 'ᓐ', 'ᓯ', 'ᓰ', 'ᓱ', 'ᓲ', 'ᓴ', 'ᓵ', 'ᔅ', 'ᓕ', 'ᓖ', 'ᓗ', 'ᓘ',
    'ᓚ', 'ᓛ', 'ᓪ', 'ᔨ', 'ᔩ', 'ᔪ', 'ᔫ', 'ᔭ', '…', ' ', 'ᔮ', 'ᔾ', 'ᕕ',
    'ᕖ', 'ᕗ', '–', '—', '“', '”', '‘', '’', 'ᕘ', 'ᕙ', 'ᕚ', 'ᕝ', 'ᕆ',
    'ᕇ', 'ᕈ', 'ᕉ', 'ᕋ', 'ᕌ', 'ᕐ', 'ᕿ', 'ᖀ', 'ᖁ', 'ᖂ', 'ᖃ', 'ᖄ', 'ᖅ',
    'ᖏ', 'ᖐ', 'ᖑ', 'ᖒ', 'ᖓ', 'ᖔ', 'ᖕ', 'ᙱ', 'ᙲ', 'ᙳ', 'ᙴ', 'ᙵ', 'ᙶ',
    'ᖖ', 'ᖠ', 'ᖡ', 'ᖢ', 'ᖣ', 'ᖤ', 'ᖥ', 'ᖦ', 'ᕼ', 'Ł', 'ł',
];

#[rustfmt::skip]
const X_MAC_ROMANIAN: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
    'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
    'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
    'ß', '®', '©', '™', '´', '¨', '≠', 'Ă', 'Ș', '∞', '±', '≤', '≥',
    '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'ă', 'ș', '¿',
    '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
    'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄',
    '€', '‹', '›', 'Ț', 'ț', '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á',
    'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
    'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ',
];

#[rustfmt::skip]
const X_MAC_TURKISH: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
    'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
    'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
    'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥',
    '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', '¿',
    '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
    'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', 'Ğ',
    'ğ', 'İ', 'ı', 'Ş', 'ş', '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á',
    'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
    '', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ',
];

pub fn decode(data: &[u8], encoding_id: EncodingID, language_id: LanguageID) -> Option<String> {
    let table = match identify(encoding_id, language_id) {
        Some(table) => table,
        _ => return None,
    };
    let mut value = String::new();
    for &byte in data {
        if byte <= 0x7F {
            value.push(byte as char);
        } else {
            value.push(table[(byte & 0x7F) as usize]);
        }
    }
    Some(value)
}

pub fn encode(
    value: &str,
    encoding_id: EncodingID,
    language_id: LanguageID,
    data: &mut Vec<u8>,
    context: &mut Context,
) -> Result<()> {
    let key = (encoding_id, language_id.into());
    let mapping = match context.mapping.get(&key) {
        Some(value) => value,
        _ => {
            let table = match identify(encoding_id, language_id) {
                Some(value) => value,
                _ => raise!("found an unknown Macintosh encoding ({encoding_id})"),
            };
            context.mapping.entry(key).or_insert(
                table
                    .iter()
                    .enumerate()
                    .map(|(index, character)| (*character, index as u8))
                    .collect(),
            )
        }
    };
    for character in value.chars() {
        if character as u16 <= 0x7F {
            data.push(character as u8);
        } else if let Some(value) = mapping.get(&character) {
            data.push(*value | 0x80);
        } else {
            raise!("found an unknown Macintosh character ({character})")
        }
    }
    Ok(())
}

fn identify(encoding_id: EncodingID, language_id: LanguageID) -> Option<&'static [char; 128]> {
    match encoding_id {
        0 => return Some(&MACINTOSH), // Roman
        // 1 => Japanese
        // 2 => Chinese (Traditional)
        // 3 => Korean
        // 4 => Arabic
        // 5 => Hebrew
        6 => return Some(&X_MAC_GREEK),    // Greek
        7 => return Some(&X_MAC_CYRILLIC), // Russian
        // 8 => RSymbol
        // 9 => Devanagari
        // 10 => Gurmukhi
        // 11 => Gujarati
        // 12 => Oriya
        // 13 => Bengali
        // 14 => Tamil
        // 15 => Telugu
        // 16 => Kannada
        // 17 => Malayalam
        // 18 => Sinhalese
        // 19 => Burmese
        // 20 => Khmer
        // 21 => Thai
        // 22 => Laotian
        // 23 => Georgian
        // 24 => Armenian
        // 25 => Chinese (Simplified)
        // 26 => Tibetan
        // 27 => Mongolian
        // 28 => Geez
        29 => return Some(&X_MAC_CE), // Slavic
        // 30 => Vietnamese
        // 31 => Sindhi
        // 32 => Uninterpreted
        _ => {}
    }
    match language_id {
        // 0 => English
        // 1 => French
        // 2 => German
        // 3 => Italian
        // 4 => Dutch
        // 5 => Swedish
        // 6 => Spanish
        // 7 => Danish
        // 8 => Portuguese
        // 9 => Norwegian
        // 10 => Hebrew
        // 11 => Japanese
        // 12 => Arabic
        // 13 => Finnish
        // 14 => Greek
        LanguageID::Macintosh(Macintosh::Icelandic) => return Some(&X_MAC_ICELANDIC), // Icelandic
        // 16 => Maltese
        LanguageID::Macintosh(Macintosh::Turkish) => return Some(&X_MAC_TURKISH), // Turkish
        LanguageID::Macintosh(Macintosh::Croatian) => return Some(&X_MAC_CROATIAN), // Croatian
        // 19 => Chinese (Traditional)
        // 20 => Urdu
        // 21 => Hindi
        // 22 => Thai
        // 23 => Korean
        LanguageID::Macintosh(Macintosh::Lithuanian) => return Some(&X_MAC_CE), // Lithuanian
        LanguageID::Macintosh(Macintosh::Polish) => return Some(&X_MAC_CE),     // Polish
        LanguageID::Macintosh(Macintosh::Hungarian) => return Some(&X_MAC_CE),  // Hungarian
        LanguageID::Macintosh(Macintosh::Estonian) => return Some(&X_MAC_CE),   // Estonian
        LanguageID::Macintosh(Macintosh::Latvian) => return Some(&X_MAC_CE),    // Latvian
        // 29 => Sami
        LanguageID::Macintosh(Macintosh::Faroese) => return Some(&X_MAC_ICELANDIC), // Faroese
        // 31 => Farsi/Persian
        // 32 => Russian
        // 33 => Chinese (Simplified)
        // 34 => Flemish
        // 35 => Irish Gaelic
        // 36 => Albanian
        LanguageID::Macintosh(Macintosh::Romanian) => return Some(&X_MAC_ROMANIAN), // Romanian
        LanguageID::Macintosh(Macintosh::Czech) => return Some(&X_MAC_CE),          // Czech
        LanguageID::Macintosh(Macintosh::Slovak) => return Some(&X_MAC_CE),         // Slovak
        LanguageID::Macintosh(Macintosh::Slovenian) => return Some(&X_MAC_CE),      // Slovenian
        // 41 => Yiddish
        // 42 => Serbian
        // 43 => Macedonian
        // 44 => Bulgarian
        // 45 => Ukrainian
        // 46 => Byelorussian
        // 47 => Uzbek
        // 48 => Kazakh
        // 49 => Azerbaijani (Cyrillic script)
        // 50 => Azerbaijani (Arabic script)
        // 51 => Armenian
        // 52 => Georgian
        // 53 => Moldavian
        // 54 => Kirghiz
        // 55 => Tajiki
        // 56 => Turkmen
        // 57 => Mongolian (Mongolian script)
        // 58 => Mongolian (Cyrillic script)
        // 59 => Pashto
        // 60 => Kurdish
        // 61 => Kashmiri
        // 62 => Sindhi
        // 63 => Tibetan
        // 64 => Nepali
        // 65 => Sanskrit
        // 66 => Marathi
        // 67 => Bengali
        // 68 => Assamese
        // 69 => Gujarati
        // 70 => Punjabi
        // 71 => Oriya
        // 72 => Malayalam
        // 73 => Kannada
        // 74 => Tamil
        // 75 => Telugu
        // 76 => Sinhalese
        // 77 => Burmese
        // 78 => Khmer
        // 79 => Lao
        // 80 => Vietnamese
        // 81 => Indonesian
        // 82 => Tagalog
        // 83 => Malay (Roman script)
        // 84 => Malay (Arabic script)
        // 85 => Amharic
        // 86 => Tigrinya
        // 87 => Galla
        // 88 => Somali
        // 89 => Swahili
        // 90 => Kinyarwanda/Ruanda
        // 91 => Rundi
        // 92 => Nyanja/Chewa
        // 93 => Malagasy
        // 94 => Esperanto
        // 128 => Welsh
        // 129 => Basque
        // 130 => Catalan
        // 131 => Latin
        // 132 => Quechua
        // 133 => Guarani
        // 134 => Aymara
        // 135 => Tatar
        // 136 => Uighur
        // 137 => Dzongkha
        // 138 => Javanese (Roman script)
        // 139 => Sundanese (Roman script)
        // 140 => Galician
        // 141 => Afrikaans
        // 142 => Breton
        LanguageID::Macintosh(Macintosh::Inuktitut) => return Some(&X_MAC_INUIT), // Inuktitut
        // 144 => Scottish Gaelic
        // 145 => Manx Gaelic
        LanguageID::Macintosh(Macintosh::IrishGaelicDot) => return Some(&X_MAC_GAELIC), // Irish Gaelic (with dot above)
        // 147 => Tongan
        // 148 => Greek (polytonic)
        // 149 => Greenlandic
        // 150 => Azerbaijani (Roman script)
        _ => {}
    }
    None
}
