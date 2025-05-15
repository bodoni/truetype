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

use crate::Result;

/// A naming table.
#[derive(Clone, Debug)]
pub enum Names {
    /// Format 0.
    Format0(Names0),
    /// Format 1.
    Format1(Names1),
}

table! {
    @position
    @write
    /// A naming table in format 0.
    pub Names0 {
        format (u16), // format
        count  (u16), // count
        offset (u16), // stringOffset

        records (Vec<Record>) |this, tape, _| { // nameRecord
            tape.take_given(this.count as usize)
        },

        data (Vec<u8>) |this, tape, position| {
            tape.jump(position + this.offset as u64)?;
            tape.take_bytes(measure(&this.records))
        },
    }
}

table! {
    @position
    @write
    /// A naming table in format 1.
    pub Names1 {
        format (u16), // format
        count  (u16), // count
        offset (u16), // stringOffset

        records (Vec<Record>) |this, tape, _| { // nameRecord
            tape.take_given(this.count as usize)
        },

        language_tag_count (u16), // langTagCount

        language_tags (Vec<LanguageTag>) |this, tape, _| { // langTagRecord
            tape.take_given(this.language_tag_count as usize)
        },

        data (Vec<u8>) |this, tape, position| {
            tape.jump(position + this.offset as u64)?;
            tape.take_bytes(measure(&this.records))
        },
    }
}

table! {
    @write
    /// A record of a naming table.
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
    @write
    /// A language tag.
    #[derive(Copy)]
    pub LanguageTag { // LangTagRecord
        size   (u16), // length
        offset (u16), // offset
    }
}

/// An encoding context.
pub type Context = encoding::macintosh::Context;

impl Names {
    /// Iterate over name records.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<
        Item = ((PlatformID, EncodingID, LanguageID, NameID), Option<String>),
    > + '_ {
        let (records, data) = match self {
            Self::Format0(ref table) => (&table.records, &table.data),
            Self::Format1(ref table) => (&table.records, &table.data),
        };
        records.iter().map(move |record| {
            let offset = record.offset as usize;
            let size = record.size as usize;
            (
                (
                    record.platform_id,
                    record.encoding_id,
                    record.language_id,
                    record.name_id,
                ),
                if cfg!(feature = "ignore-invalid-name-records") {
                    if offset + size <= data.len() {
                        decode(
                            record.platform_id,
                            record.encoding_id,
                            record.language_id,
                            &data[offset..(offset + size)],
                        )
                    } else {
                        None
                    }
                } else {
                    decode(
                        record.platform_id,
                        record.encoding_id,
                        record.language_id,
                        &data[offset..(offset + size)],
                    )
                },
            )
        })
    }

    /// Create an instance from an iterator over name records.
    pub fn from_iter<T, U, V, W>(
        records: T,
        language_tags: U,
        context: &mut Context,
    ) -> Result<Self>
    where
        T: IntoIterator<Item = ((PlatformID, EncodingID, LanguageID, NameID), V)>,
        U: IntoIterator<Item = W>,
        V: AsRef<str>,
        W: AsRef<str>,
    {
        let mut data = vec![];
        let records = records
            .into_iter()
            .filter_map(
                |((platform_id, encoding_id, language_id, name_id), value)| {
                    let offset = data.len();
                    let result = encode(
                        platform_id,
                        encoding_id,
                        language_id,
                        value.as_ref(),
                        &mut data,
                        context,
                    );
                    if cfg!(feature = "ignore-invalid-name-records") {
                        if result.is_err() {
                            return None;
                        }
                    } else if let Err(error) = result {
                        return Some(Err(error));
                    }
                    Some(Ok(Record {
                        platform_id,
                        encoding_id,
                        language_id,
                        name_id,
                        size: (data.len() - offset) as _,
                        offset: offset as _,
                    }))
                },
            )
            .collect::<Result<Vec<_>>>()?;
        let language_tags = language_tags
            .into_iter()
            .map(|value| {
                let offset = data.len();
                encoding::unicode::encode_utf16(value.as_ref(), &mut data);
                LanguageTag {
                    size: (data.len() - offset) as _,
                    offset: offset as _,
                }
            })
            .collect::<Vec<_>>();
        let table = if language_tags.is_empty() {
            Self::Format0(Names0 {
                format: 0,
                count: records.len() as _,
                offset: (2 * (3 + records.len() * 6)) as _,
                records,
                data,
            })
        } else {
            Self::Format1(Names1 {
                format: 1,
                count: records.len() as _,
                offset: (2 * (3 + records.len() * 6 + 1 + language_tags.len() * 2)) as _,
                records,
                language_tag_count: language_tags.len() as _,
                language_tags,
                data,
            })
        };
        Ok(table)
    }

    /// Iterate over the language tags.
    pub fn language_tags(&self) -> impl DoubleEndedIterator<Item = Option<String>> + '_ {
        let (records, data) = match self {
            Self::Format0(ref table) => (&[][..], &table.data),
            Self::Format1(ref table) => (&table.language_tags[..], &table.data),
        };
        records.iter().map(|record| {
            let offset = record.offset as usize;
            let size = record.size as usize;
            encoding::unicode::decode_utf16(&data[offset..(offset + size)])
        })
    }
}

impl crate::value::Read for Names {
    fn read<T: crate::tape::Read>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u16>()? {
            0 => Self::Format0(tape.take()?),
            1 => Self::Format1(tape.take()?),
            _ => raise!("found an unknown format of the naming table"),
        })
    }
}

impl crate::value::Write for Names {
    fn write<T: crate::tape::Write>(&self, tape: &mut T) -> Result<()> {
        match self {
            Self::Format0(value) => tape.give(value),
            Self::Format1(value) => tape.give(value),
        }
    }
}

fn decode(
    platform_id: PlatformID,
    encoding_id: EncodingID,
    language_id: LanguageID,
    data: &[u8],
) -> Option<String> {
    match platform_id {
        PlatformID::Unicode => encoding::unicode::decode(data, encoding_id),
        PlatformID::Macintosh => encoding::macintosh::decode(data, encoding_id, language_id),
        PlatformID::Windows => encoding::windows::decode(data, encoding_id),
    }
}

fn encode(
    platform_id: PlatformID,
    encoding_id: EncodingID,
    language_id: LanguageID,
    value: &str,
    data: &mut Vec<u8>,
    context: &mut encoding::macintosh::Context,
) -> Result<()> {
    match platform_id {
        PlatformID::Unicode => encoding::unicode::encode(value, encoding_id, data),
        PlatformID::Macintosh => {
            encoding::macintosh::encode(value, encoding_id, language_id, data, context)
        }
        PlatformID::Windows => encoding::windows::encode(value, encoding_id, data),
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
