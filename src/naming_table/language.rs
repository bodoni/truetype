//! The languages.

use crate::naming_table::platform::PlatformID;
use crate::{Result, Tape, Walue};

/// A language identifier.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LanguageID {
    #[default]
    Unicode,
    Macintosh(Macintosh),
    Windows(Windows),
    Other(usize),
}

// Reference:
// https://github.com/opentypejs/opentype.js/blob/c37fcdfbd89c1bd0aac1cecb2b287dfb7d00cee0/src/tables/name.js#L35-L155
choices! {
    #[doc = "A Macintosh language."]
    pub Macintosh(u16) {
        0 => English ("en"), // English
        1 => French ("fr"), // French
        2 => German ("de"), // German
        3 => Italian ("it"), // Italian
        4 => Dutch ("nl"), // Dutch
        5 => Swedish ("sv"), // Swedish
        6 => Spanish ("es"), // Spanish
        7 => Danish ("da"), // Danish
        8 => Portuguese ("pt"), // Portuguese
        9 => Norwegian ("no"), // Norwegian
        10 => Hebrew ("he"), // Hebrew
        11 => Japanese ("ja"), // Japanese
        12 => Arabic ("ar"), // Arabic
        13 => Finnish ("fi"), // Finnish
        14 => Greek ("el"), // Greek
        15 => Icelandic ("is"), // Icelandic
        16 => Maltese ("mt"), // Maltese
        17 => Turkish ("tr"), // Turkish
        18 => Croatian ("hr"), // Croatian
        19 => ChineseTraditional ("zh-Hant"), // Chinese (Traditional)
        20 => Urdu ("ur"), // Urdu
        21 => Hindi ("hi"), // Hindi
        22 => Thai ("th"), // Thai
        23 => Korean ("ko"), // Korean
        24 => Lithuanian ("lt"), // Lithuanian
        25 => Polish ("pl"), // Polish
        26 => Hungarian ("hu"), // Hungarian
        27 => Estonian ("es"), // Estonian
        28 => Latvian ("lv"), // Latvian
        29 => Sami ("se"), // Sami
        30 => Faroese ("fo"), // Faroese
        31 => FarsiPersian ("fa"), // Farsi/Persian
        32 => Russian ("ru"), // Russian
        33 => ChineseSimplified ("zh"), // Chinese (Simplified)
        34 => Flemish ("nl-BE"), // Flemish
        35 => IrishGaelic ("ga"), // Irish Gaelic
        36 => Albanian ("sq"), // Albanian
        37 => Romanian ("ro"), // Romanian
        38 => Czech ("cz"), // Czech
        39 => Slovak ("sk"), // Slovak
        40 => Slovenian ("si"), // Slovenian
        41 => Yiddish ("yi"), // Yiddish
        42 => Serbian ("sr"), // Serbian
        43 => Macedonian ("mk"), // Macedonian
        44 => Bulgarian ("bg"), // Bulgarian
        45 => Ukrainian ("uk"), // Ukrainian
        46 => Byelorussian ("be"), // Byelorussian
        47 => Uzbek ("uz"), // Uzbek
        48 => Kazakh ("kk"), // Kazakh
        49 => AzerbaijaniCyrillic ("az-Cyrl"), // Azerbaijani (Cyrillic script)
        50 => AzerbaijaniArabic ("az-Arab"), // Azerbaijani (Arabic script)
        51 => Armenian ("hy"), // Armenian
        52 => Georgian ("ka"), // Georgian
        53 => Moldavian ("mo"), // Moldavian
        54 => Kirghiz ("ky"), // Kirghiz
        55 => Tajiki ("tg"), // Tajiki
        56 => Turkmen ("tk"), // Turkmen
        57 => MongolianMongolian ("mn-CN"), // Mongolian (Mongolian script)
        58 => MongolianCyrillic ("mn"), // Mongolian (Cyrillic script)
        59 => Pashto ("ps"), // Pashto
        60 => Kurdish ("ks"), // Kurdish
        61 => Kashmiri ("ku"), // Kashmiri
        62 => Sindhi ("sd"), // Sindhi
        63 => Tibetan ("bo"), // Tibetan
        64 => Nepali ("ne"), // Nepali
        65 => Sanskrit ("sa"), // Sanskrit
        66 => Marathi ("mr"), // Marathi
        67 => Bengali ("bn"), // Bengali
        68 => Assamese ("as"), // Assamese
        69 => Gujarati ("gu"), // Gujarati
        70 => Punjabi ("pa"), // Punjabi
        71 => Oriya ("or"), // Oriya
        72 => Malayalam ("ml"), // Malayalam
        73 => Kannada ("kn"), // Kannada
        74 => Tamil ("ta"), // Tamil
        75 => Telugu ("te"), // Telugu
        76 => Sinhalese ("si"), // Sinhalese
        77 => Burmese ("my"), // Burmese
        78 => Khmer ("km"), // Khmer
        79 => Lao ("lo"), // Lao
        80 => Vietnamese ("vi"), // Vietnamese
        81 => Indonesian ("id"), // Indonesian
        82 => Tagalog ("tl"), // Tagalog
        83 => MalayRoman ("ms"), // Malay (Roman script)
        84 => MalayArabic ("ms-Arab"), // Malay (Arabic script)
        85 => Amharic ("am"), // Amharic
        86 => Tigrinya ("ti"), // Tigrinya
        87 => Galla ("om"), // Galla
        88 => Somali ("so"), // Somali
        89 => Swahili ("sw"), // Swahili
        90 => KinyarwandaRuanda ("rw"), // Kinyarwanda/Ruanda
        91 => Rundi ("rn"), // Rundi
        92 => NyanjaChewa ("ny"), // Nyanja/Chewa
        93 => Malagasy ("mg"), // Malagasy
        94 => Esperanto ("eo"), // Esperanto
        128 => Welsh ("cy"), // Welsh
        129 => Basque ("eu"), // Basque
        130 => Catalan ("ca"), // Catalan
        131 => Latin ("la"), // Latin
        132 => Quechua ("qu"), // Quechua
        133 => Guarani ("gn"), // Guarani
        134 => Aymara ("ay"), // Aymara
        135 => Tatar ("tt"), // Tatar
        136 => Uighur ("ug"), // Uighur
        137 => Dzongkha ("dz"), // Dzongkha
        138 => JavaneseRoman ("jv"), // Javanese (Roman script)
        139 => SundaneseRoman ("su"), // Sundanese (Roman script)
        140 => Galician ("gl"), // Galician
        141 => Afrikaans ("af"), // Afrikaans
        142 => Breton ("br"), // Breton
        143 => Inuktitut ("iu"), // Inuktitut
        144 => ScottishGaelic ("gd"), // Scottish Gaelic
        145 => ManxGaelic ("gv"), // Manx Gaelic
        146 => IrishGaelicDot ("ga"), // Irish Gaelic (with dot above)
        147 => Tongan ("to"), // Tongan
        148 => GreekPolytonic ("el-polyton"), // Greek (polytonic)
        149 => Greenlandic ("kl"), // Greenlandic
        150 => AzerbaijaniRoman ("az"), // Azerbaijani (Roman script)
    }
}

// Reference:
// https://github.com/opentypejs/opentype.js/blob/c37fcdfbd89c1bd0aac1cecb2b287dfb7d00cee0/src/tables/name.js#L307-L522
choices! {
    #[doc = "A Windows language."]
    pub Windows(u16) {
        0x0436 => AfrikaansSouthAfrica ("af"), // Afrikaans, South Africa
        0x041C => AlbanianAlbania ("sq"), // Albanian, Albania
        0x0484 => AlsatianFrance ("gsw"), // Alsatian, France
        0x045E => AmharicEthiopia ("am"), // Amharic, Ethiopia
        0x1401 => ArabicAlgeria ("ar-DZ"), // Arabic, Algeria
        0x3C01 => ArabicBahrain ("ar-BH"), // Arabic, Bahrain
        0x0C01 => ArabicEgypt ("ar"), // Arabic, Egypt
        0x0801 => ArabicIraq ("ar-IQ"), // Arabic, Iraq
        0x2C01 => ArabicJordan ("ar-JO"), // Arabic, Jordan
        0x3401 => ArabicKuwait ("ar-KW"), // Arabic, Kuwait
        0x3001 => ArabicLebanon ("ar-LB"), // Arabic, Lebanon
        0x1001 => ArabicLibya ("ar-LY"), // Arabic, Libya
        0x1801 => ArabicMorocco ("ary"), // Arabic, Morocco
        0x2001 => ArabicOman ("ar-OM"), // Arabic, Oman
        0x4001 => ArabicQatar ("ar-QA"), // Arabic, Qatar
        0x0401 => ArabicSaudiArabia ("ar-SA"), // Arabic, Saudi Arabia
        0x2801 => ArabicSyria ("ar-SY"), // Arabic, Syria
        0x1C01 => ArabicTunisia ("aeb"), // Arabic, Tunisia
        0x3801 => ArabicUAE ("ar-AE"), // Arabic, U.A.E.
        0x2401 => ArabicYemen ("ar-YE"), // Arabic, Yemen
        0x042B => ArmenianArmenia ("hy"), // Armenian, Armenia
        0x044D => AssameseIndia ("as"), // Assamese, India
        0x082C => AzeriCyrillicAzerbaijan ("az-Cyrl"), // Azeri (Cyrillic), Azerbaijan
        0x042C => AzeriLatinAzerbaijan ("az"), // Azeri (Latin), Azerbaijan
        0x046D => BashkirRussia ("ba"), // Bashkir, Russia
        0x042D => BasqueBasque ("eu"), // Basque, Basque
        0x0423 => BelarusianBelarus ("be"), // Belarusian, Belarus
        0x0845 => BengaliBangladesh ("bn"), // Bengali, Bangladesh
        0x0445 => BengaliIndia ("bn-IN"), // Bengali, India
        0x201A => BosnianCyrillicBosniaHerzegovina ("bs-Cyrl"), // Bosnian (Cyrillic), Bosnia and Herzegovina
        0x141A => BosnianLatinBosniaHerzegovina ("bs"), // Bosnian (Latin), Bosnia and Herzegovina
        0x047E => BretonFrance ("br"), // Breton, France
        0x0402 => BulgarianBulgaria ("bg"), // Bulgarian, Bulgaria
        0x0403 => CatalanCatalan ("ca"), // Catalan, Catalan
        0x0C04 => ChineseHongKongSAR ("zh-HK"), // Chinese, Hong Kong S.A.R.
        0x1404 => ChineseMacaoSAR ("zh-MO"), // Chinese, Macao S.A.R.
        0x0804 => ChineseChina ("zh"), // Chinese, People’s Republic of China
        0x1004 => ChineseSingapore ("zh-SG"), // Chinese, Singapore
        0x0404 => ChineseTaiwan ("zh-TW"), // Chinese, Taiwan
        0x0483 => CorsicanFrance ("co"), // Corsican, France
        0x041A => CroatianCroatia ("hr"), // Croatian, Croatia
        0x101A => CroatianLatinBosniaHerzegovina ("hr-BA"), // Croatian (Latin), Bosnia and Herzegovina
        0x0405 => CzechCzechRepublic ("cs"), // Czech, Czech Republic
        0x0406 => DanishDenmark ("da"), // Danish, Denmark
        0x048C => DariAfghanistan ("prs"), // Dari, Afghanistan
        0x0465 => DivehiMaldives ("dv"), // Divehi, Maldives
        0x0813 => DutchBelgium ("nl-BE"), // Dutch, Belgium
        0x0413 => DutchNetherlands ("nl"), // Dutch, Netherlands
        0x0C09 => EnglishAustralia ("en-AU"), // English, Australia
        0x2809 => EnglishBelize ("en-BZ"), // English, Belize
        0x1009 => EnglishCanada ("en-CA"), // English, Canada
        0x2409 => EnglishCaribbean ("en-029"), // English, Caribbean
        0x4009 => EnglishIndia ("en-IN"), // English, India
        0x1809 => EnglishIreland ("en-IE"), // English, Ireland
        0x2009 => EnglishJamaica ("en-JM"), // English, Jamaica
        0x4409 => EnglishMalaysia ("en-MY"), // English, Malaysia
        0x1409 => EnglishNewZealand ("en-NZ"), // English, New Zealand
        0x3409 => EnglishPhilippines ("en-PH"), // English, Republic of the Philippines
        0x4809 => EnglishSingapore ("en-SG"), // English, Singapore
        0x1C09 => EnglishSouthAfrica ("en-ZA"), // English, South Africa
        0x2C09 => EnglishTrinidadTobago ("en-TT"), // English, Trinidad and Tobago
        0x0809 => EnglishUnitedKingdom ("en-GB"), // English, United Kingdom
        0x0409 => EnglishUnitedStates ("en"), // English, United States
        0x3009 => EnglishZimbabwe ("en-ZW"), // English, Zimbabwe
        0x0425 => EstonianEstonia ("et"), // Estonian, Estonia
        0x0438 => FaroeseFaroeIslands ("fo"), // Faroese, Faroe Islands
        0x0464 => FilipinoPhilippines ("fil"), // Filipino, Philippines
        0x040B => FinnishFinland ("fi"), // Finnish, Finland
        0x080C => FrenchBelgium ("fr-BE"), // French, Belgium
        0x0C0C => FrenchCanada ("fr-CA"), // French, Canada
        0x040C => FrenchFrance ("fr"), // French, France
        0x140C => FrenchLuxembourg ("fr-LU"), // French, Luxembourg
        0x180C => FrenchMonaco ("fr-MC"), // French, Principality of Monaco
        0x100C => FrenchSwitzerland ("fr-CH"), // French, Switzerland
        0x0462 => FrisianNetherlands ("fy"), // Frisian, Netherlands
        0x0456 => GalicianGalician ("gl"), // Galician, Galician
        0x0437 => GeorgianGeorgia ("ka"), // Georgian, Georgia
        0x0C07 => GermanAustria ("de-AT"), // German, Austria
        0x0407 => GermanGermany ("de"), // German, Germany
        0x1407 => GermanLiechtenstein ("de-LI"), // German, Liechtenstein
        0x1007 => GermanLuxembourg ("de-LU"), // German, Luxembourg
        0x0807 => GermanSwitzerland ("de-CH"), // German, Switzerland
        0x0408 => GreekGreece ("el"), // Greek, Greece
        0x046F => GreenlandicGreenland ("kl"), // Greenlandic, Greenland
        0x0447 => GujaratiIndia ("gu"), // Gujarati, India
        0x0468 => HausaLatinNigeria ("ha"), // Hausa (Latin), Nigeria
        0x040D => HebrewIsrael ("he"), // Hebrew, Israel
        0x0439 => HindiIndia ("hi"), // Hindi, India
        0x040E => HungarianHungary ("hu"), // Hungarian, Hungary
        0x040F => IcelandicIceland ("is"), // Icelandic, Iceland
        0x0470 => IgboNigeria ("ig"), // Igbo, Nigeria
        0x0421 => IndonesianIndonesia ("id"), // Indonesian, Indonesia
        0x045D => InuktitutCanada ("iu"), // Inuktitut, Canada
        0x085D => InuktitutLatinCanada ("iu-Latn"), // Inuktitut (Latin), Canada
        0x083C => IrishIreland ("ga"), // Irish, Ireland
        0x0434 => IsiXhosaSouthAfrica ("xh"), // isiXhosa, South Africa
        0x0435 => IsiZuluSouthAfrica ("zu"), // isiZulu, South Africa
        0x0410 => ItalianItaly ("it"), // Italian, Italy
        0x0810 => ItalianSwitzerland ("it-CH"), // Italian, Switzerland
        0x0411 => JapaneseJapan ("ja"), // Japanese, Japan
        0x044B => KannadaIndia ("kn"), // Kannada, India
        0x043F => KazakhKazakhstan ("kk"), // Kazakh, Kazakhstan
        0x0453 => KhmerCambodia ("km"), // Khmer, Cambodia
        0x0486 => KicheGuatemala ("quc"), // K’iche, Guatemala
        0x0487 => KinyarwandaRwanda ("rw"), // Kinyarwanda, Rwanda
        0x0441 => KiswahiliKenya ("sw"), // Kiswahili, Kenya
        0x0457 => KonkaniIndia ("kok"), // Konkani, India
        0x0412 => KoreanKorea ("ko"), // Korean, Korea
        0x0440 => KyrgyzKyrgyzstan ("ky"), // Kyrgyz, Kyrgyzstan
        0x0454 => LaoLaoPDR ("lo"), // Lao, Lao P.D.R.
        0x0426 => LatvianLatvia ("lv"), // Latvian, Latvia
        0x0427 => LithuanianLithuania ("lt"), // Lithuanian, Lithuania
        0x082E => LowerSorbianGermany ("dsb"), // Lower Sorbian, Germany
        0x046E => LuxembourgishLuxembourg ("lb"), // Luxembourgish, Luxembourg
        0x042F => MacedonianNorthMacedonia ("mk"), // Macedonian, North Macedonia
        0x083E => MalayBruneiDarussalam ("ms-BN"), // Malay, Brunei Darussalam
        0x043E => MalayMalaysia ("ms"), // Malay, Malaysia
        0x044C => MalayalamIndia ("ml"), // Malayalam, India
        0x043A => MalteseMalta ("mt"), // Maltese, Malta
        0x0481 => MaoriNewZealand ("mi"), // Maori, New Zealand
        0x047A => MapudungunChile ("arn"), // Mapudungun, Chile
        0x044E => MarathiIndia ("mr"), // Marathi, India
        0x047C => MohawkMohawk ("moh"), // Mohawk, Mohawk
        0x0450 => MongolianCyrillicMongolia ("mn"), // Mongolian (Cyrillic), Mongolia
        0x0850 => MongolianTraditionalChina ("mn-CN"), // Mongolian (Traditional), People’s Republic of China
        0x0461 => NepaliNepal ("ne"), // Nepali, Nepal
        0x0414 => NorwegianBokmalNorway ("nb"), // Norwegian (Bokmal), Norway
        0x0814 => NorwegianNynorskNorway ("nn"), // Norwegian (Nynorsk), Norway
        0x0482 => OccitanFrance ("oc"), // Occitan, France
        0x0448 => OdiaformerlyOriyaIndia ("or"), // Odia (formerly Oriya), India
        0x0463 => PashtoAfghanistan ("ps"), // Pashto, Afghanistan
        0x0415 => PolishPoland ("pl"), // Polish, Poland
        0x0416 => PortugueseBrazil ("pt"), // Portuguese, Brazil
        0x0816 => PortuguesePortugal ("pt-PT"), // Portuguese, Portugal
        0x0446 => PunjabiIndia ("pa"), // Punjabi, India
        0x046B => QuechuaBolivia ("qu-BO"), // Quechua, Bolivia
        0x086B => QuechuaEcuador ("qu-EC"), // Quechua, Ecuador
        0x0C6B => QuechuaPeru ("qu"), // Quechua, Peru
        0x0418 => RomanianRomania ("ro"), // Romanian, Romania
        0x0417 => RomanshSwitzerland ("rm"), // Romansh, Switzerland
        0x0419 => RussianRussia ("ru"), // Russian, Russia
        0x243B => SamiInariFinland ("smn"), // Sami (Inari), Finland
        0x103B => SamiLuleNorway ("smj-NO"), // Sami (Lule), Norway
        0x143B => SamiLuleSweden ("smj"), // Sami (Lule), Sweden
        0x0C3B => SamiNorthernFinland ("se-FI"), // Sami (Northern), Finland
        0x043B => SamiNorthernNorway ("se"), // Sami (Northern), Norway
        0x083B => SamiNorthernSweden ("se-SE"), // Sami (Northern), Sweden
        0x203B => SamiSkoltFinland ("sms"), // Sami (Skolt), Finland
        0x183B => SamiSouthernNorway ("sma-NO"), // Sami (Southern), Norway
        0x1C3B => SamiSouthernSweden ("sms"), // Sami (Southern), Sweden
        0x044F => SanskritIndia ("sa"), // Sanskrit, India
        0x1C1A => SerbianCyrillicBosniaHerzegovina ("sr-Cyrl-BA"), // Serbian (Cyrillic), Bosnia and Herzegovina
        0x0C1A => SerbianCyrillicSerbia ("sr"), // Serbian (Cyrillic), Serbia
        0x181A => SerbianLatinBosniaHerzegovina ("sr-Latn-BA"), // Serbian (Latin), Bosnia and Herzegovina
        0x081A => SerbianLatinSerbia ("sr-Latn"), // Serbian (Latin), Serbia
        0x046C => SesothoSaLeboaSouthAfrica ("nso"), // Sesotho sa Leboa, South Africa
        0x0432 => SetswanaSouthAfrica ("tn"), // Setswana, South Africa
        0x045B => SinhalaSriLanka ("si"), // Sinhala, Sri Lanka
        0x041B => SlovakSlovakia ("sk"), // Slovak, Slovakia
        0x0424 => SlovenianSlovenia ("sl"), // Slovenian, Slovenia
        0x2C0A => SpanishArgentina ("es-AR"), // Spanish, Argentina
        0x400A => SpanishBolivia ("es-BO"), // Spanish, Bolivia
        0x340A => SpanishChile ("es-CL"), // Spanish, Chile
        0x240A => SpanishColombia ("es-CO"), // Spanish, Colombia
        0x140A => SpanishCostaRica ("es-CR"), // Spanish, Costa Rica
        0x1C0A => SpanishDominicanRepublic ("es-DO"), // Spanish, Dominican Republic
        0x300A => SpanishEcuador ("es-EC"), // Spanish, Ecuador
        0x440A => SpanishElSalvador ("es-SV"), // Spanish, El Salvador
        0x100A => SpanishGuatemala ("es-GT"), // Spanish, Guatemala
        0x480A => SpanishHonduras ("es-HN"), // Spanish, Honduras
        0x080A => SpanishMexico ("es-MX"), // Spanish, Mexico
        0x4C0A => SpanishNicaragua ("es-NI"), // Spanish, Nicaragua
        0x180A => SpanishPanama ("es-PA"), // Spanish, Panama
        0x3C0A => SpanishParaguay ("es-PY"), // Spanish, Paraguay
        0x280A => SpanishPeru ("es-PE"), // Spanish, Peru
        0x500A => SpanishPuertoRico ("es-PR"), // Spanish, Puerto Rico
        0x0C0A => SpanishModernSpain ("es"), // Spanish (Modern Sort), Spain
        0x040A => SpanishTraditionalSpain ("es-ES-u-co-trad"), // Spanish (Traditional Sort), Spain
        0x540A => SpanishUnitedStates ("es-US"), // Spanish, United States
        0x380A => SpanishUruguay ("es-UY"), // Spanish, Uruguay
        0x200A => SpanishVenezuela ("es-VE"), // Spanish, Venezuela
        0x081D => SwedishFinland ("sv-FI"), // Swedish, Finland
        0x041D => SwedishSweden ("sv"), // Swedish, Sweden
        0x045A => SyriacSyria ("syr"), // Syriac, Syria
        0x0428 => TajikCyrillicTajikistan ("tg"), // Tajik (Cyrillic), Tajikistan
        0x085F => TamazightLatinAlgeria ("tzm"), // Tamazight (Latin), Algeria
        0x0449 => TamilIndia ("ta"), // Tamil, India
        0x0444 => TatarRussia ("tt"), // Tatar, Russia
        0x044A => TeluguIndia ("te"), // Telugu, India
        0x041E => ThaiThailand ("th"), // Thai, Thailand
        0x0451 => TibetanPRC ("bo"), // Tibetan, PRC
        0x041F => TurkishTurkey ("tr"), // Turkish, Turkey
        0x0442 => TurkmenTurkmenistan ("tk"), // Turkmen, Turkmenistan
        0x0480 => UighurPRC ("ug"), // Uighur, PRC
        0x0422 => UkrainianUkraine ("uk"), // Ukrainian, Ukraine
        0x042E => UpperSorbianGermany ("hsb"), // Upper Sorbian, Germany
        0x0420 => UrduIslamicPakistan ("ur"), // Urdu, Islamic Republic of Pakistan
        0x0843 => UzbekCyrillicUzbekistan ("uz-Cyrl"), // Uzbek (Cyrillic), Uzbekistan
        0x0443 => UzbekLatinUzbekistan ("uz"), // Uzbek (Latin), Uzbekistan
        0x042A => VietnameseVietnam ("vi"), // Vietnamese, Vietnam
        0x0452 => WelshUnitedKingdom ("cy"), // Welsh, United Kingdom
        0x0488 => WolofSenegal ("wo"), // Wolof, Senegal
        0x0485 => YakutRussia ("sah"), // Yakut, Russia
        0x0478 => YiPRC ("ii"), // Yi, PRC
        0x046A => YorubaNigeria ("yo"), // Yoruba, Nigeria
    }
}

impl Walue<'static> for LanguageID {
    type Parameter = PlatformID;

    fn read<T: Tape>(tape: &mut T, platform_id: PlatformID) -> Result<Self> {
        match (platform_id, tape.take::<u16>()?) {
            (PlatformID::Unicode, _) => Ok(LanguageID::Unicode),
            (PlatformID::Macintosh, value) if value < 0x8000 => {
                Ok(LanguageID::Macintosh(value.try_into()?))
            }
            (PlatformID::Windows, value) if value < 0x8000 => {
                Ok(LanguageID::Windows(value.try_into()?))
            }
            (_, value) => Ok(LanguageID::Other(value as usize - 0x8000)),
        }
    }
}
