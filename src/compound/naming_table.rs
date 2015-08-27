use std::mem;

use Result;
use tape::{Tape, Value};

/// A naming table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NamingTable {
    /// Format 0.
    Format0(NamingTable0),
    /// Format 1.
    Format1(NamingTable1),
}

table! {
    #[doc = "A naming table of format 0."]
    pub NamingTable0 {
        format       (u16            ),
        count        (u16            ),
        stringOffset (u16            ),
        nameRecord   (Vec<NameRecord>) |tape, this| { read_vector!(tape, this.count) },
        storage      (Vec<u8>        ) |tape, this| { this.read_storage(tape) },
    }
}

table! {
    #[doc = "A naming table of format 1."]
    pub NamingTable1 {
        format        (u16                   ),
        count         (u16                   ),
        stringOffset  (u16                   ),
        nameRecord    (Vec<NameRecord>       ) |tape, this| { read_vector!(tape, this.count) },
        langTagCount  (u16                   ),
        langTagRecord (Vec<LanguageTagRecord>) |tape, this| { read_vector!(tape,
                                                                           this.langTagCount) },
        storage       (Vec<u8>               ) |tape, this| { this.read_storage(tape) },
    }
}

table! {
    #[doc = "A name record of a naming table."]
    #[derive(Copy)]
    #[repr(C)]
    pub NameRecord {
        platformID (u16),
        encodingID (u16),
        languageID (u16),
        nameID     (u16),
        length     (u16),
        offset     (u16),
    }
}

table! {
    #[doc = "A language-tag record of a naming table."]
    #[derive(Copy)]
    #[repr(C)]
    pub LanguageTagRecord {
        length (u16),
        offset (u16),
    }
}

impl Value for NamingTable {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match try!(tape.peek::<u16>()) {
            0 => NamingTable::Format0(try!(Value::read(tape))),
            1 => NamingTable::Format1(try!(Value::read(tape))),
            _ => raise!("the format of the naming table is not supported"),
        })
    }
}

impl NamingTable0 {
    #[inline]
    pub fn strings(&self) -> Result<Vec<String>> {
        strings(&self.nameRecord, &self.storage)
    }

    fn read_storage<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = try!(tape.position());
        let above = 3 * 2 + self.nameRecord.len() * mem::size_of::<NameRecord>();
        try!(tape.jump(current - above as u64 + self.stringOffset as u64));
        read_vector!(tape, storage_length(&self.nameRecord), u8)
    }
}

impl NamingTable1 {
    #[inline]
    pub fn strings(&self) -> Result<Vec<String>> {
        strings(&self.nameRecord, &self.storage)
    }

    fn read_storage<T: Tape>(&self, tape: &mut T) -> Result<Vec<u8>> {
        let current = try!(tape.position());
        let above = 4 * 2 + self.nameRecord.len() * mem::size_of::<NameRecord>() +
                            self.langTagRecord.len() * mem::size_of::<LanguageTagRecord>();
        try!(tape.jump(current - above as u64 + self.stringOffset as u64));
        read_vector!(tape, storage_length(&self.nameRecord), u8)
    }
}

fn storage_length(records: &[NameRecord]) -> usize {
    let mut length = 0;
    for record in records {
        let end = record.offset + record.length + 1;
        if end > length {
            length = end;
        }
    }
    length as usize
}

fn strings(records: &[NameRecord], storage: &[u8]) -> Result<Vec<String>> {
    let mut strings = vec![];
    for record in records {
        let (offset, length) = (record.offset as usize, record.length as usize);
        let bytes = &storage[offset..(offset + length)];
        match record.platformID {
            1 => match decode_macintosh(bytes, record.encodingID) {
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
fn decode_macintosh(bytes: &[u8], encoding: u16) -> Option<String> {
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

    if encoding != 0 {
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
