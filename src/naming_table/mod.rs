//! The [naming table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/name

use std::mem;

use crate::{Result, Tape, Value};

mod encoding;

/// A name identifier.
pub type NameID = u16;

/// A naming table.
#[derive(Clone, Debug)]
pub enum NamingTable {
    /// Format 0.
    Format0(NamingTable0),
    /// Format 1.
    Format1(NamingTable1),
}

table! {
    #[doc = "A naming table in format 0."]
    pub NamingTable0 {
        format (u16), // format
        count  (u16), // count
        offset (u16), // stringOffset

        records (Vec<Record>) |this, tape| { // nameRecord
            tape.take_given(this.count as usize)
        },

        data (Vec<u8>) |this, tape| {
            this.read_data(tape)
        },
    }
}

table! {
    #[doc = "A naming table in format 1."]
    pub NamingTable1 {
        format (u16), // format
        count  (u16), // count
        offset (u16), // stringOffset

        records (Vec<Record>) |this, tape| { // nameRecord
            tape.take_given(this.count as usize)
        },

        language_count (u16), // langTagCount

        languages (Vec<Language>) |this, tape| { // langTagRecord
            tape.take_given(this.language_count as usize)
        },

        data (Vec<u8>) |this, tape| {
            this.read_data(tape)
        },
    }
}

table! {
    #[doc = "A record of a naming table."]
    #[derive(Copy)]
    #[repr(C)]
    pub Record { // NameRecord
        platform_id (u16   ), // platformID
        encoding_id (u16   ), // encodingID
        language_id (u16   ), // languageID
        name_id     (NameID), // nameID
        length      (u16   ), // length
        offset      (u16   ), // offset
    }
}

table! {
    #[doc = "A language-tag record of a naming table."]
    #[derive(Copy)]
    #[repr(C)]
    pub Language { // LangTagRecord
        length (u16), // length
        offset (u16), // offset
    }
}

/// A predefined name.
#[derive(Clone, Copy, Debug)]
pub enum PredefinedName {
    CopyrightNotice = 0,
    FontFamilyName = 1,
    FontSubfamilyName = 2,
    UniqueFontID = 3,
    FullFontName = 4,
    VersionString = 5,
    PostScriptFontName = 6,
    Trademark = 7,
    ManufacturerName = 8,
    DesignerName = 9,
    Description = 10,
    VendorURL = 11,
    DesignerURL = 12,
    LicenseDescription = 13,
    LicenseURL = 14,
    // Reserved = 15,
    TypographicFamilyName = 16,
    TypographicSubfamilyName = 17,
    CompatibleFullFontName = 18,
    SampleText = 19,
    PostScriptCIDFindFontName = 20,
    WWSFamilyName = 21,
    WWSSubfamilyName = 22,
    LightBackgroundPalette = 23,
    DarkBackgroundPalette = 24,
    PostScriptVariationNamePrefix = 25,
}

impl NamingTable {
    /// Search and decode a specific name.
    pub fn get<T: Into<NameID>>(&self, name_id: T) -> Option<String> {
        match self {
            &NamingTable::Format0(ref table) => get(&table.records, &table.data, name_id.into()),
            &NamingTable::Format1(ref table) => get(&table.records, &table.data, name_id.into()),
        }
    }

    /// Decode all names.
    pub fn get_all(&self) -> Vec<(NameID, Option<String>)> {
        match self {
            &NamingTable::Format0(ref table) => get_all(&table.records, &table.data),
            &NamingTable::Format1(ref table) => get_all(&table.records, &table.data),
        }
    }
}

impl Value for NamingTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            0 => NamingTable::Format0(tape.take()?),
            1 => NamingTable::Format1(tape.take()?),
            _ => raise!("found an unknown format of the naming table"),
        })
    }
}

impl NamingTable0 {
    fn read_data<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = tape.position()?;
        let above = 3 * 2 + self.records.len() * mem::size_of::<Record>();
        tape.jump(current - above as u64 + self.offset as u64)?;
        tape.take_bytes(compute_length(&self.records))
    }
}

impl NamingTable1 {
    fn read_data<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = tape.position()?;
        let above = 4 * 2
            + self.records.len() * mem::size_of::<Record>()
            + self.languages.len() * mem::size_of::<Language>();
        tape.jump(current - above as u64 + self.offset as u64)?;
        tape.take_bytes(compute_length(&self.records))
    }
}

impl From<PredefinedName> for NameID {
    #[inline]
    fn from(name: PredefinedName) -> NameID {
        name as NameID
    }
}

fn compute_length(records: &[Record]) -> usize {
    let mut length = 0;
    for record in records {
        let end = record.offset + record.length;
        if end > length {
            length = end;
        }
    }
    length as usize
}

#[inline]
fn decode(record: &Record, data: &[u8]) -> Option<String> {
    match record.platform_id {
        0 => encoding::unicode::decode(data, record.encoding_id),
        1 => encoding::macintosh::decode(data, record.encoding_id, record.language_id),
        3 => encoding::windows::decode(data, record.encoding_id),
        _ => None,
    }
}

fn get(records: &[Record], data: &[u8], name_id: NameID) -> Option<String> {
    for record in records {
        if record.name_id != name_id {
            continue;
        }
        let (offset, length) = (record.offset as usize, record.length as usize);
        let string = decode(record, &data[offset..(offset + length)]);
        if string.is_none() {
            continue;
        }
        return string;
    }
    None
}

fn get_all(records: &[Record], data: &[u8]) -> Vec<(NameID, Option<String>)> {
    let mut names = vec![];
    for record in records {
        let (offset, length) = (record.offset as usize, record.length as usize);
        let string = decode(record, &data[offset..(offset + length)]);
        names.push((record.name_id, string));
    }
    names
}
