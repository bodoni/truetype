//! The [naming table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/name

mod encoding;
mod name;
mod platform;

pub mod language;

use std::mem;

use crate::{Result, Tape, Value};

pub use encoding::EncodingID;
pub use language::LanguageID;
pub use name::{Name, NameID};
pub use platform::{Platform, PlatformID};

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

        language_tag_count (u16), // langTagCount

        language_tags (Vec<LanguageTag>) |this, tape| { // langTagRecord
            tape.take_given(this.language_tag_count as usize)
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
        platform_id (PlatformID), // platformID
        encoding_id (EncodingID), // encodingID
        language_id (LanguageID), // languageID
        name_id     (NameID    ), // nameID
        length      (u16       ), // length
        offset      (u16       ), // offset
    }
}

table! {
    #[doc = "A language-tag record of a naming table."]
    #[derive(Copy)]
    #[repr(C)]
    pub LanguageTag { // LangTagRecord
        length (u16), // length
        offset (u16), // offset
    }
}

impl NamingTable {
    /// Decode all records.
    pub fn decode(&self) -> Vec<(NameID, Option<String>, Option<String>)> {
        let (records, language_tags, data) = match self {
            &NamingTable::Format0(ref table) => (&table.records, &[][..], &table.data),
            &NamingTable::Format1(ref table) => {
                (&table.records, &table.language_tags[..], &table.data)
            }
        };
        let language_tags: Vec<_> = language_tags
            .iter()
            .map(|record| {
                let (offset, length) = (record.offset as usize, record.length as usize);
                encoding::unicode::decode_utf16(&data[offset..(offset + length)])
            })
            .collect();
        records
            .iter()
            .map(|record| {
                let language_tag = record.language_tag(&language_tags);
                let (offset, length) = (record.offset as usize, record.length as usize);
                let string = decode(
                    record.platform_id,
                    record.encoding_id,
                    record.language_id,
                    language_tag.as_ref(),
                    &data[offset..(offset + length)],
                );
                (record.name_id, language_tag, string)
            })
            .collect()
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
            + self.language_tags.len() * mem::size_of::<LanguageTag>();
        tape.jump(current - above as u64 + self.offset as u64)?;
        tape.take_bytes(compute_length(&self.records))
    }
}

impl Record {
    /// Return the IETF-BCP-47 language.
    pub fn language_tag(&self, language_tags: &[Option<String>]) -> Option<String> {
        if self.language_id < 0x8000 {
            match self.platform_id {
                1 => match language::Macintosh::try_from(self.language_id) {
                    Ok(language) => Some(<&'static str>::from(language).into()),
                    _ => None,
                },
                3 => match language::Windows::try_from(self.language_id) {
                    Ok(language) => Some(<&'static str>::from(language).into()),
                    _ => None,
                },
                _ => None,
            }
        } else {
            match language_tags.get((self.language_id - 0x8000) as usize) {
                Some(Some(language)) => Some(language.clone()),
                _ => None,
            }
        }
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

fn decode(
    platform_id: PlatformID,
    encoding_id: EncodingID,
    language_id: LanguageID,
    language_tag: Option<&String>,
    data: &[u8],
) -> Option<String> {
    match platform_id {
        0 => encoding::unicode::decode(data, encoding_id),
        1 => encoding::macintosh::decode(data, encoding_id, language_id, language_tag),
        3 => encoding::windows::decode(data, encoding_id),
        _ => None,
    }
}
