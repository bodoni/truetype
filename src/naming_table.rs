//! The [naming table][1].
//!
//! [1]: https://www.microsoft.com/typography/otspec/name.htm

use std::mem;

use {Result, Tape, Value, Walue};

/// A naming table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NamingTable {
    /// Format 0.
    Format0(Format0),
    /// Format 1.
    Format1(Format1),
}

table! {
    #[doc = "A naming table in format 0."]
    pub Format0 {
        format (u16), // format
        count  (u16), // count
        offset (u16), // stringOffset

        records (Vec<Record>) |tape, this| { // nameRecord
            Walue::read(tape, this.count as usize)
        },

        data (Vec<u8>) |tape, this| {
            this.read_data(tape)
        },
    }
}

table! {
    #[doc = "A naming table in format 1."]
    pub Format1 {
        format (u16), // format
        count  (u16), // count
        offset (u16), // stringOffset

        records (Vec<Record>) |tape, this| { // nameRecord
            Walue::read(tape, this.count as usize)
        },

        language_count (u16), // langTagCount

        languages (Vec<Language>) |tape, this| { // langTagRecord
            Walue::read(tape, this.language_count as usize)
        },

        data (Vec<u8>) |tape, this| {
            this.read_data(tape)
        },
    }
}

table! {
    #[doc = "A record of a naming table."]
    #[derive(Copy)]
    #[repr(C)]
    pub Record { // NameRecord
        platform_id (u16), // platformID
        encoding_id (u16), // encodingID
        language_id (u16), // languageID
        name_id     (u16), // nameID
        length      (u16), // length
        offset      (u16), // offset
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

impl Value for NamingTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            0 => NamingTable::Format0(try!(tape.take())),
            1 => NamingTable::Format1(try!(tape.take())),
            _ => raise!("the format of the naming table is not supported"),
        })
    }
}

impl Format0 {
    #[inline]
    pub fn strings(&self) -> Result<Vec<String>> {
        strings(&self.records, &self.data)
    }

    fn read_data<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = try!(tape.position());
        let above = 3 * 2 + self.records.len() * mem::size_of::<Record>();
        try!(tape.jump(current - above as u64 + self.offset as u64));
        Ok(read_bytes!(tape, data_length(&self.records)))
    }
}

impl Format1 {
    #[inline]
    pub fn strings(&self) -> Result<Vec<String>> {
        strings(&self.records, &self.data)
    }

    fn read_data<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = try!(tape.position());
        let above = 4 * 2 + self.records.len() * mem::size_of::<Record>() +
                            self.languages.len() * mem::size_of::<Language>();
        try!(tape.jump(current - above as u64 + self.offset as u64));
        Ok(read_bytes!(tape, data_length(&self.records)))
    }
}

fn data_length(records: &[Record]) -> usize {
    let mut length = 0;
    for record in records {
        let end = record.offset + record.length + 1;
        if end > length {
            length = end;
        }
    }
    length as usize
}

fn strings(records: &[Record], data: &[u8]) -> Result<Vec<String>> {
    let mut strings = vec![];
    for record in records {
        let (offset, length) = (record.offset as usize, record.length as usize);
        let bytes = &data[offset..(offset + length)];
        match record.platform_id {
            1 => match decode_macintosh(bytes, record.encoding_id) {
                Some(string) => {
                    strings.push(string);
                    continue;
                },
                _ => {},
            },
            _ => {},
        }
        strings.push("<unsupported>".to_string());
    }
    Ok(strings)
}

// The implementation is based on
// https://github.com/nodebox/opentype.js/blob/master/src/types.js#L300
fn decode_macintosh(bytes: &[u8], encoding_id: u16) -> Option<String> {
    const ROMAN: [char; 128] = ['Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å',
                                'ç', 'é', 'è', 'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô',
                                'ö', 'õ', 'ú', 'ù', 'û', 'ü', '†', '°', '¢', '£', '§', '•', '¶',
                                'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', '∞', '±', '≤', '≥',
                                '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', '¿',
                                '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', ' ', 'À', 'Ã', 'Õ',
                                'Œ', 'œ', '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄',
                                '€', '‹', '›', 'ﬁ', 'ﬂ', '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á',
                                'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', '', 'Ò', 'Ú', 'Û', 'Ù',
                                'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ'];

    if encoding_id != 0 {
        return None;
    }

    let table = &ROMAN;

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
