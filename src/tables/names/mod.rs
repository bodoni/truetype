//! The [naming table][1].
//!
//! [1]: https://learn.microsoft.com/en-us/typography/opentype/spec/name

pub mod language;

mod encoding;
mod name;
mod platform;

pub use encoding::EncodingID;
pub use language::LanguageID;
pub use name::NameID;
pub use platform::PlatformID;

use crate::{Result, Tape, Value};

/// A naming table.
#[derive(Clone, Debug)]
pub enum Names {
    /// Format 0.
    Format0(Names0),
    /// Format 1.
    Format1(Names1),
}

table! {
    #[doc = "A naming table in format 0."]
    pub Names0 {
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
    pub Names1 {
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
    pub Record { // NameRecord
        platform_id (PlatformID), // platformID
        encoding_id (EncodingID), // encodingID

        language_id (LanguageID) |this, tape| { // languageID
            tape.take_given(this.platform_id)
        },

        name_id     (NameID), // nameID
        size        (u16   ), // length
        offset      (u16   ), // offset
    }
}

table! {
    #[doc = "A language tag."]
    #[derive(Copy)]
    pub LanguageTag { // LangTagRecord
        size   (u16), // length
        offset (u16), // offset
    }
}

/// A type iterating over name entries.
pub trait Entries:
    Iterator<Item = ((NameID, Option<String>), Option<String>)> + DoubleEndedIterator
{
}

impl Names {
    /// Iterate over all name entires.
    ///
    /// Each entry is represented by three quantities: a name ID and a language tag, which are
    /// given as a tuple, and the corresponding value.
    pub fn iter(&self) -> impl Entries + '_ {
        let (records, language_tags, data) = match self {
            Names::Format0(ref table) => (&table.records, &[][..], &table.data),
            Names::Format1(ref table) => (&table.records, &table.language_tags[..], &table.data),
        };
        let language_tags: Vec<_> = language_tags
            .iter()
            .map(|record| {
                let (offset, size) = (record.offset as usize, record.size as usize);
                encoding::unicode::decode_utf16(&data[offset..(offset + size)])
            })
            .collect();
        records.iter().map(move |record| {
            let language_tag = record.language_tag(&language_tags);
            let (offset, size) = (record.offset as usize, record.size as usize);
            let value = decode(
                record.platform_id,
                record.encoding_id,
                record.language_id,
                language_tag.as_deref(),
                &data[offset..(offset + size)],
            );
            ((record.name_id, language_tag), value)
        })
    }
}

impl Value for Names {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            0 => Names::Format0(tape.take()?),
            1 => Names::Format1(tape.take()?),
            _ => raise!("found an unknown format of the naming table"),
        })
    }
}

impl Names0 {
    fn read_data<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = tape.position()?;
        let above = 3 * 2 + self.records.len() * 6 * 2;
        tape.jump(current - above as u64 + self.offset as u64)?;
        tape.take_bytes(measure(&self.records))
    }
}

impl Names1 {
    fn read_data<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = tape.position()?;
        let above = 4 * 2 + self.records.len() * 6 * 2 + self.language_tags.len() * 2 * 2;
        tape.jump(current - above as u64 + self.offset as u64)?;
        tape.take_bytes(measure(&self.records))
    }
}

impl Record {
    /// Return the IETF-BCP-47 language tag.
    pub fn language_tag(&self, language_tags: &[Option<String>]) -> Option<String> {
        match self.language_id {
            LanguageID::Unicode => None,
            LanguageID::Macintosh(value) => Some(<&'static str>::from(value).into()),
            LanguageID::Windows(value) => Some(<&'static str>::from(value).into()),
            LanguageID::Other(value) => match language_tags.get(value) {
                Some(Some(value)) => Some(value.clone()),
                _ => None,
            },
            #[cfg(feature = "ignore-invalid-language-ids")]
            LanguageID::Invalid(_) => None,
        }
    }
}

impl<T> Entries for T where
    T: Iterator<Item = ((NameID, Option<String>), Option<String>)> + DoubleEndedIterator
{
}

fn decode(
    platform_id: PlatformID,
    encoding_id: EncodingID,
    language_id: LanguageID,
    language_tag: Option<&str>,
    data: &[u8],
) -> Option<String> {
    match platform_id {
        PlatformID::Unicode => encoding::unicode::decode(data, encoding_id),
        PlatformID::Macintosh => {
            encoding::macintosh::decode(data, encoding_id, language_id, language_tag)
        }
        PlatformID::Windows => encoding::windows::decode(data, encoding_id),
    }
}

fn measure(records: &[Record]) -> usize {
    let mut size = 0;
    for record in records {
        let end = record.offset + record.size;
        if end > size {
            size = end;
        }
    }
    size as usize
}