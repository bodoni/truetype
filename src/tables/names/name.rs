choices! {
    /// A name identifier.
    pub NameID(u16) {
        0 => CopyrightNotice,
        1 => FontFamilyName,
        2 => FontSubfamilyName,
        3 => UniqueFontID,
        4 => FullFontName,
        5 => VersionString,
        6 => PostScriptFontName,
        7 => Trademark,
        8 => ManufacturerName,
        9 => DesignerName,
        10 => Description,
        11 => VendorURL,
        12 => DesignerURL,
        13 => LicenseDescription,
        14 => LicenseURL,
        // 15 => Reserved,
        16 => TypographicFamilyName,
        17 => TypographicSubfamilyName,
        18 => CompatibleFullFontName,
        19 => SampleText,
        20 => PostScriptCIDFindFontName,
        21 => WWSFamilyName,
        22 => WWSSubfamilyName,
        23 => LightBackgroundPalette,
        24 => DarkBackgroundPalette,
        25 => PostScriptVariationNamePrefix,
        _ => Other,
    }
}
