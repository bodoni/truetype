//! The [OS/2 and Windows metrics][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/os2.htm

use {Result, Tape, Value};

/// OS/2 and Windows metrics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowsMetrics {
    /// Version 3.
    Version3(Version3),
    /// Version 5.
    Version5(Version5),
}

macro_rules! read_flags(
    ($tape:ident, $kind:ty) => ({
        let value = try!($tape.take::<$kind>());
        if value.is_invalid() {
            raise!("the OS/2 and Windows metrics table is corrupted");
        }
        Ok(value)
    });
);

table! {
    #[doc = "OS/2 and Windows metrics of version 3."]
    pub Version3 {
        version               (u16), // version
        average_char_width    (i16), // xAvgCharWidth
        weight_class          (u16), // usWeightClass
        width_class           (u16), // usWidthClass

        type_flags (TypeFlags) |tape, this| { // fsType
            read_flags!(tape, TypeFlags)
        },

        subscript_x_size      (i16     ), // ySubscriptXSize
        subscript_y_size      (i16     ), // ySubscriptYSize
        subscript_x_offset    (i16     ), // ySubscriptXOffset
        subscript_y_offset    (i16     ), // ySubscriptYOffset
        superscript_x_size    (i16     ), // ySuperscriptXSize
        superscript_y_size    (i16     ), // ySuperscriptYSize
        superscript_x_offset  (i16     ), // ySuperscriptXOffset
        superscript_y_offset  (i16     ), // ySuperscriptYOffset
        strikeout_size        (i16     ), // yStrikeoutSize
        strikeout_position    (i16     ), // yStrikeoutPosition
        family_class          (i16     ), // sFamilyClass
        panose                ([u8; 10]), // panose
        unicode_range1        (u32     ), // ulUnicodeRange1
        unicode_range2        (u32     ), // ulUnicodeRange2
        unicode_range3        (u32     ), // ulUnicodeRange3
        unicode_range4        (u32     ), // ulUnicodeRange4
        vendor_id             ([i8; 4] ), // achVendID

        selection_flags (SelectionFlags) |tape, this| { // fsSelection
            read_flags!(tape, SelectionFlags)
        },

        first_char_index      (u16), // usFirstCharIndex
        last_char_index       (u16), // usLastCharIndex
        typographic_ascender  (i16), // sTypoAscender
        typographic_descender (i16), // sTypoDescender
        typographic_line_gap  (i16), // sTypoLineGap
        windows_ascender      (u16), // usWinAscent
        windows_descender     (u16), // usWinDescent
        code_page_range1      (u32), // ulCodePageRange1
        code_page_range2      (u32), // ulCodePageRange2
        x_height              (i16), // sxHeight
        cap_height            (i16), // sCapHeight
        default_char          (u16), // usDefaultChar
        break_char            (u16), // usBreakChar
        max_context           (u16), // usMaxContext
    }
}

table! {
    #[doc = "OS/2 and Windows metrics of version 5."]
    pub Version5 {
        version               (u16), // version
        average_char_width    (i16), // xAvgCharWidth
        weight_class          (u16), // usWeightClass
        width_class           (u16), // usWidthClass

        type_flags (TypeFlags) |tape, this| { // fsType
            read_flags!(tape, TypeFlags)
        },

        subscript_x_size      (i16     ), // ySubscriptXSize
        subscript_y_size      (i16     ), // ySubscriptYSize
        subscript_x_offset    (i16     ), // ySubscriptXOffset
        subscript_y_offset    (i16     ), // ySubscriptYOffset
        superscript_x_size    (i16     ), // ySuperscriptXSize
        superscript_y_size    (i16     ), // ySuperscriptYSize
        superscript_x_offset  (i16     ), // ySuperscriptXOffset
        superscript_y_offset  (i16     ), // ySuperscriptYOffset
        strikeout_size        (i16     ), // yStrikeoutSize
        strikeout_position    (i16     ), // yStrikeoutPosition
        family_class          (i16     ), // sFamilyClass
        panose                ([u8; 10]), // panose
        unicode_range1        (u32     ), // ulUnicodeRange1
        unicode_range2        (u32     ), // ulUnicodeRange2
        unicode_range3        (u32     ), // ulUnicodeRange3
        unicode_range4        (u32     ), // ulUnicodeRange4
        vendor_id             ([i8; 4] ), // achVendID

        selection_flags (SelectionFlags) |tape, this| { // fsSelection
            read_flags!(tape, SelectionFlags)
        },

        first_char_index      (u16), // usFirstCharIndex
        last_char_index       (u16), // usLastCharIndex
        typographic_ascender  (i16), // sTypoAscender
        typographic_descender (i16), // sTypoDescender
        typographic_line_gap  (i16), // sTypoLineGap
        windows_ascender      (u16), // usWinAscent
        windows_descender     (u16), // usWinDescent
        code_page_range1      (u32), // ulCodePageRange1
        code_page_range2      (u32), // ulCodePageRange2
        x_height              (i16), // sxHeight
        cap_height            (i16), // sCapHeight
        default_char          (u16), // usDefaultChar
        break_char            (u16), // usBreakChar
        max_context           (u16), // usMaxContext

        lower_optical_point_size (u16), // usLowerOpticalPointSize
        upper_optical_point_size (u16), // usUpperOpticalPointSize
    }
}

flags! {
    #[doc = "Type flags."]
    pub TypeFlags(u16) {
        0b1111_1100_0000_0000 => is_invalid,
    }
}

flags! {
    #[doc = "Font selection flags."]
    pub SelectionFlags(u16) {
        0b1111_1100_0000_0000 => is_invalid,
    }
}

impl Value for WindowsMetrics {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            3 => WindowsMetrics::Version3(try!(tape.take())),
            5 => WindowsMetrics::Version5(try!(tape.take())),
            _ => raise!("the format of the OS/2 and Windows metrics is not supported"),
        })
    }
}
