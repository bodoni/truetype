use Result;
use tape::{Tape, Value};

/// OS/2 and Windows metrics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowsMetrics {
    /// Version 3.
    Version3(WindowsMetrics3),
    /// Version 5.
    Version5(WindowsMetrics5),
}

table! {
    #[doc = "OS/2 and Windows metrics of version 3."]
    pub WindowsMetrics3 {
        version             (u16    ),
        xAvgCharWidth       (i16    ),
        usWeightClass       (u16    ),
        usWidthClass        (u16    ),
        fsType              (u16    ),
        ySubscriptXSize     (i16    ),
        ySubscriptYSize     (i16    ),
        ySubscriptXOffset   (i16    ),
        ySubscriptYOffset   (i16    ),
        ySuperscriptXSize   (i16    ),
        ySuperscriptYSize   (i16    ),
        ySuperscriptXOffset (i16    ),
        ySuperscriptYOffset (i16    ),
        yStrikeoutSize      (i16    ),
        yStrikeoutPosition  (i16    ),
        sFamilyClass        (i16    ),
        panose              (Vec<u8>) |tape, this| { read_vector!(tape, 10, u8) },
        ulUnicodeRange1     (u32    ),
        ulUnicodeRange2     (u32    ),
        ulUnicodeRange3     (u32    ),
        ulUnicodeRange4     (u32    ),
        achVendID           (Vec<i8>) |tape, this| { read_vector!(tape, 4, i8) },
        fsSelection         (u16    ),
        usFirstCharIndex    (u16    ),
        usLastCharIndex     (u16    ),
        sTypoAscender       (i16    ),
        sTypoDescender      (i16    ),
        sTypoLineGap        (i16    ),
        usWinAscent         (u16    ),
        usWinDescent        (u16    ),
        ulCodePageRange1    (u32    ),
        ulCodePageRange2    (u32    ),
        sxHeight            (i16    ),
        sCapHeight          (i16    ),
        usDefaultChar       (u16    ),
        usBreakChar         (u16    ),
        usMaxContext        (u16    ),
    }
}

table! {
    #[doc = "OS/2 and Windows metrics of version 5."]
    pub WindowsMetrics5 {
        version                 (u16    ),
        xAvgCharWidth           (i16    ),
        usWeightClass           (u16    ),
        usWidthClass            (u16    ),
        fsType                  (u16    ),
        ySubscriptXSize         (i16    ),
        ySubscriptYSize         (i16    ),
        ySubscriptXOffset       (i16    ),
        ySubscriptYOffset       (i16    ),
        ySuperscriptXSize       (i16    ),
        ySuperscriptYSize       (i16    ),
        ySuperscriptXOffset     (i16    ),
        ySuperscriptYOffset     (i16    ),
        yStrikeoutSize          (i16    ),
        yStrikeoutPosition      (i16    ),
        sFamilyClass            (i16    ),
        panose                  (Vec<u8>) |tape, this| { read_vector!(tape, 10, u8) },
        ulUnicodeRange1         (u32    ),
        ulUnicodeRange2         (u32    ),
        ulUnicodeRange3         (u32    ),
        ulUnicodeRange4         (u32    ),
        achVendID               (Vec<i8>) |tape, this| { read_vector!(tape, 4, i8) },
        fsSelection             (u16    ),
        usFirstCharIndex        (u16    ),
        usLastCharIndex         (u16    ),
        sTypoAscender           (i16    ),
        sTypoDescender          (i16    ),
        sTypoLineGap            (i16    ),
        usWinAscent             (u16    ),
        usWinDescent            (u16    ),
        ulCodePageRange1        (u32    ),
        ulCodePageRange2        (u32    ),
        sxHeight                (i16    ),
        sCapHeight              (i16    ),
        usDefaultChar           (u16    ),
        usBreakChar             (u16    ),
        usMaxContext            (u16    ),
        usLowerOpticalPointSize (u16    ),
        usUpperOpticalPointSize (u16    ),
    }
}

impl Value for WindowsMetrics {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            3 => WindowsMetrics::Version3(try!(Value::read(tape))),
            5 => WindowsMetrics::Version5(try!(Value::read(tape))),
            _ => raise!("the format of the OS/2 and Windows metrics is not supported"),
        })
    }
}
