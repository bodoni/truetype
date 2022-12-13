// Reference:
// https://github.com/opentypejs/opentype.js/blob/c37fcdfbd89c1bd0aac1cecb2b287dfb7d00cee0/src/types.js#L463-L482

#[rustfmt::skip]
const ENCODING_GREEK: [char; 128] = [
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

#[rustfmt::skip]
const ENCODING_RUSSIAN: [char; 128] = [
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
const ENCODING_SLAVIC: [char; 128] = [
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

pub fn decode(bytes: &[u8], encoding_id: u16, language_id: u16) -> Option<String> {
    let table = match (encoding_id, language_id) {
        (0, _) => &ROMAN, // Roman
        // 1 => Japanese
        // 2 => Chinese (Traditional)
        // 3 => Korean
        // 4 => Arabic
        // 5 => Hebrew
        (6, _) => &ENCODING_GREEK,   // Greek
        (7, _) => &ENCODING_RUSSIAN, // Russian
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
        (29, _) => &ENCODING_SLAVIC, // Slavic
        // 30 => Vietnamese
        // 31 => Sindhi
        // 32 => Uninterpreted

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
        // 15 => Icelandic
        // 16 => Maltese
        // 17 => Turkish
        // 18 => Croatian
        // 19 => Chinese (Traditional)
        // 20 => Urdu
        // 21 => Hindi
        // 22 => Thai
        // 23 => Korean
        (_, 24) => &ENCODING_SLAVIC, // Lithuanian
        (_, 25) => &ENCODING_SLAVIC, // Polish
        (_, 26) => &ENCODING_SLAVIC, // Hungarian
        (_, 27) => &ENCODING_SLAVIC, // Estonian
        (_, 28) => &ENCODING_SLAVIC, // Latvian
        // 29 => Sami
        // 30 => Faroese
        // 31 => Farsi/Persian
        // 32 => Russian
        // 33 => Chinese (Simplified)
        // 34 => Flemish
        // 35 => Irish Gaelic
        // 36 => Albanian
        // 37 => Romanian
        (_, 38) => &ENCODING_SLAVIC, // Czech
        (_, 39) => &ENCODING_SLAVIC, // Slovak
        (_, 40) => &ENCODING_SLAVIC, // Slovenian
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
        // 143 => Inuktitut
        // 144 => Scottish Gaelic
        // 145 => Manx Gaelic
        // 146 => Irish Gaelic (with dot above)
        // 147 => Tongan
        // 148 => Greek (polytonic)
        // 149 => Greenlandic
        // 150 => Azerbaijani (Roman script)
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
