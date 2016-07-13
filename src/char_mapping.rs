//! Char-to-glyph mapping.

use std::collections::HashMap;

use {Result, Tape, Value};

/// A char-to-glyph mapping.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CharMapping {
    pub header: Header,
    pub records: Vec<Record>,
    pub encodings: Vec<Encoding>,
}

/// An encoding of a char-to-glyph mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Encoding {
    /// Format 4.
    Format4(EncodingFormat4),
    /// Format 6.
    Format6(EncodingFormat6),
}

macro_rules! read_version(
    ($tape:ident) => ({
        let value = try!(Value::read($tape));
        if value != 0 {
            raise!("the version of the char-to-glyph mapping header is not supported");
        }
        Ok(value)
    });
);

table! {
    #[doc = "The header of a char-to-glyph mapping."]
    #[derive(Copy)]
    pub Header {
        version (u16) |tape, this| { // version
            read_version!(tape)
        },

        table_count (u16), // numTables
    }
}

table! {
    #[doc = "A record of a char-to-glyph mapping."]
    #[derive(Copy)]
    pub Record {
        platform_id (u16), // platformID
        encoding_id (u16), // encodingID
        offset      (u32), // offset
    }
}

table! {
    #[doc = "A char-to-glyph encoding of format 4."]
    pub EncodingFormat4 {
        format           (u16), // format
        length           (u16), // length
        language         (u16), // language
        segment_count_x2 (u16), // segCountX2
        search_range     (u16), // searchRange
        entry_selector   (u16), // entrySelector
        range_shift      (u16), // rangeShift

        end_codes (Vec<u16>) |tape, this| { // endCode
            read_vector!(tape, this.segment_count())
        },

        reserved_pad (u16), // reservedPad

        start_codes (Vec<u16>) |tape, this| { // startCode
            read_vector!(tape, this.segment_count())
        },

        id_deltas (Vec<i16>) |tape, this| { // idDelta
            read_vector!(tape, this.segment_count())
        },

        id_range_offsets (Vec<u16>) |tape, this| { // idRangeOffset
            read_vector!(tape, this.segment_count())
        },

        glyph_indices (Vec<u16>) |tape, this| { // glyphIdArray
            read_vector!(tape, try!(this.array_length()))
        },
    }
}

table! {
    #[doc = "A char-to-glyph encoding of format 6."]
    pub EncodingFormat6 {
        format      (u16), // format
        length      (u16), // length
        language    (u16), // language
        first_code  (u16), // firstCode
        entry_count (u16), // entryCount

        glyph_indices (Vec<u16>) |tape, this| { // glyphIdArray
            read_vector!(tape, this.entry_count)
        },
    }
}

impl Value for CharMapping {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let header = match try!(tape.peek::<u16>()) {
            0 => try!(Header::read(tape)),
            _ => raise!("the format of the char-to-glyph mapping header is not supported"),
        };
        let mut records = vec![];
        for _ in 0..header.table_count {
            records.push(try!(Record::read(tape)));
        }
        let mut encodings = vec![];
        for encoding in records.iter() {
            try!(tape.jump(position + encoding.offset as u64));
            encodings.push(match try!(tape.peek::<u16>()) {
                4 => Encoding::Format4(try!(Value::read(tape))),
                6 => Encoding::Format6(try!(Value::read(tape))),
                _ => unimplemented!(),
            });
        }
        Ok(CharMapping { header: header, records: records, encodings: encodings })
    }
}

impl Encoding {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        match self {
            &Encoding::Format4(ref encoding) => encoding.mapping(),
            &Encoding::Format6(ref encoding) => encoding.mapping(),
        }
    }
}

impl EncodingFormat4 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        let count = self.segment_count();
        let mut map = HashMap::new();
        for i in 0..(count - 1) {
            let start_code = self.start_codes[i];
            let id_delta = self.id_deltas[i];
            let id_range_offset = self.id_range_offsets[i];
            for j in start_code..(self.end_codes[i] + 1) {
                let index = if id_range_offset > 0 {
                    let offset = (id_range_offset / 2 + (j - start_code)) - (count - i) as u16;
                    self.glyph_indices[offset as usize]
                } else {
                    (id_delta + j as i16) as u16
                };
                map.insert(j, index);
            }
        }
        map
    }

    fn array_length(&self) -> Result<usize> {
        let count = self.segment_count();
        if count == 0 {
            raise!("found a char-to-glyph mapping with no segments");
        }
        if self.start_codes[count - 1] != 0xffff || self.end_codes[count - 1] != 0xffff {
            raise!("found a malformed char-to-glyph mapping");
        }
        let mut length = 0;
        for i in 0..(count - 1) {
            let start_code = self.start_codes[i];
            let id_range_offset = self.id_range_offsets[i];
            for j in start_code..(self.end_codes[i] + 1) {
                if id_range_offset > 0 {
                    let end = (id_range_offset / 2 + (j - start_code)) - (count - i) as u16 + 1;
                    if end > length {
                        length = end;
                    }
                }
            }
        }
        Ok(length as usize)
    }

    #[inline]
    fn segment_count(&self) -> usize {
        self.segment_count_x2 as usize / 2
    }
}

impl EncodingFormat6 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        unimplemented!();
    }
}
