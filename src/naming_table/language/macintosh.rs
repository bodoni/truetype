//! The Macintosh languages.

use crate::naming_table::LanguageID;

/// A Macintosh language.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Language {
    English = 0, // English
    French = 1, // French
    German = 2, // German
    Italian = 3, // Italian
    Dutch = 4, // Dutch
    Swedish = 5, // Swedish
    Spanish = 6, // Spanish
    Danish = 7, // Danish
    Portuguese = 8, // Portuguese
    Norwegian = 9, // Norwegian
    Hebrew = 10, // Hebrew
    Japanese = 11, // Japanese
    Arabic = 12, // Arabic
    Finnish = 13, // Finnish
    Greek = 14, // Greek
    Icelandic = 15, // Icelandic
    Maltese = 16, // Maltese
    Turkish = 17, // Turkish
    Croatian = 18, // Croatian
    ChineseTraditional = 19, // Chinese (Traditional)
    Urdu = 20, // Urdu
    Hindi = 21, // Hindi
    Thai = 22, // Thai
    Korean = 23, // Korean
    Lithuanian = 24, // Lithuanian
    Polish = 25, // Polish
    Hungarian = 26, // Hungarian
    Estonian = 27, // Estonian
    Latvian = 28, // Latvian
    Sami = 29, // Sami
    Faroese = 30, // Faroese
    FarsiPersian = 31, // Farsi/Persian
    Russian = 32, // Russian
    ChineseSimplified = 33, // Chinese (Simplified)
    Flemish = 34, // Flemish
    IrishGaelic = 35, // Irish Gaelic
    Albanian = 36, // Albanian
    Romanian = 37, // Romanian
    Czech = 38, // Czech
    Slovak = 39, // Slovak
    Slovenian = 40, // Slovenian
    Yiddish = 41, // Yiddish
    Serbian = 42, // Serbian
    Macedonian = 43, // Macedonian
    Bulgarian = 44, // Bulgarian
    Ukrainian = 45, // Ukrainian
    Byelorussian = 46, // Byelorussian
    Uzbek = 47, // Uzbek
    Kazakh = 48, // Kazakh
    AzerbaijaniCyrillic = 49, // Azerbaijani (Cyrillic script)
    AzerbaijaniArabic = 50, // Azerbaijani (Arabic script)
    Armenian = 51, // Armenian
    Georgian = 52, // Georgian
    Moldavian = 53, // Moldavian
    Kirghiz = 54, // Kirghiz
    Tajiki = 55, // Tajiki
    Turkmen = 56, // Turkmen
    MongolianMongolian = 57, // Mongolian (Mongolian script)
    MongolianCyrillic = 58, // Mongolian (Cyrillic script)
    Pashto = 59, // Pashto
    Kurdish = 60, // Kurdish
    Kashmiri = 61, // Kashmiri
    Sindhi = 62, // Sindhi
    Tibetan = 63, // Tibetan
    Nepali = 64, // Nepali
    Sanskrit = 65, // Sanskrit
    Marathi = 66, // Marathi
    Bengali = 67, // Bengali
    Assamese = 68, // Assamese
    Gujarati = 69, // Gujarati
    Punjabi = 70, // Punjabi
    Oriya = 71, // Oriya
    Malayalam = 72, // Malayalam
    Kannada = 73, // Kannada
    Tamil = 74, // Tamil
    Telugu = 75, // Telugu
    Sinhalese = 76, // Sinhalese
    Burmese = 77, // Burmese
    Khmer = 78, // Khmer
    Lao = 79, // Lao
    Vietnamese = 80, // Vietnamese
    Indonesian = 81, // Indonesian
    Tagalog = 82, // Tagalog
    MalayRoman = 83, // Malay (Roman script)
    MalayArabic = 84, // Malay (Arabic script)
    Amharic = 85, // Amharic
    Tigrinya = 86, // Tigrinya
    Galla = 87, // Galla
    Somali = 88, // Somali
    Swahili = 89, // Swahili
    KinyarwandaRuanda = 90, // Kinyarwanda/Ruanda
    Rundi = 91, // Rundi
    NyanjaChewa = 92, // Nyanja/Chewa
    Malagasy = 93, // Malagasy
    Esperanto = 94, // Esperanto
    Welsh = 128, // Welsh
    Basque = 129, // Basque
    Catalan = 130, // Catalan
    Latin = 131, // Latin
    Quechua = 132, // Quechua
    Guarani = 133, // Guarani
    Aymara = 134, // Aymara
    Tatar = 135, // Tatar
    Uighur = 136, // Uighur
    Dzongkha = 137, // Dzongkha
    JavaneseRoman = 138, // Javanese (Roman script)
    SundaneseRoman = 139, // Sundanese (Roman script)
    Galician = 140, // Galician
    Afrikaans = 141, // Afrikaans
    Breton = 142, // Breton
    Inuktitut = 143, // Inuktitut
    ScottishGaelic = 144, // Scottish Gaelic
    ManxGaelic = 145, // Manx Gaelic
    IrishGaelicDot = 146, // Irish Gaelic (with dot above)
    Tongan = 147, // Tongan
    GreekPolytonic = 148, // Greek (polytonic)
    Greenlandic = 149, // Greenlandic
    AzerbaijaniRoman = 150, // Azerbaijani (Roman script)
}

impl From<Language> for LanguageID {
    #[inline]
    fn from(language: Language) -> LanguageID {
        language as LanguageID
    }
}
