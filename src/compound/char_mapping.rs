use std::collections::HashMap;

use Result;
use tape::{Tape, Value};

/// A char-to-glyph mapping.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CharMapping {
    pub header: CharMappingHeader,
    pub records: Vec<CharMappingRecord>,
    pub encodings: Vec<CharMappingEncoding>,
}

/// An encoding of a char-to-glyph mapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharMappingEncoding {
    Format4(CharMappingEncoding4),
    Format6(CharMappingEncoding6),
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
    pub CharMappingHeader {
        version   (u16) |tape, this| { read_version!(tape) },
        numTables (u16),
    }
}

table! {
    #[doc = "A record of a char-to-glyph mapping."]
    #[derive(Copy)]
    pub CharMappingRecord {
        platformID (u16),
        encodingID (u16),
        offset     (u32),
    }
}

table! {
    #[doc = "A char-to-glyph encoding of format 4."]
    pub CharMappingEncoding4 {
        format        (u16     ),
        length        (u16     ),
        language      (u16     ),
        segCountX2    (u16     ),
        searchRange   (u16     ),
        entrySelector (u16     ),
        rangeShift    (u16     ),
        endCode       (Vec<u16>) |tape, this| { read_vector!(tape, this.segments()) },
        reservedPad   (u16     ),
        startCode     (Vec<u16>) |tape, this| { read_vector!(tape, this.segments()) },
        idDelta       (Vec<i16>) |tape, this| { read_vector!(tape, this.segments()) },
        idRangeOffset (Vec<u16>) |tape, this| { read_vector!(tape, this.segments()) },
        glyphIdArray  (Vec<u16>) |tape, this| { read_vector!(tape, try!(this.array_length())) },
    }
}

table! {
    #[doc = "A char-to-glyph encoding of format 6."]
    pub CharMappingEncoding6 {
        format       (u16     ),
        length       (u16     ),
        language     (u16     ),
        firstCode    (u16     ),
        entryCount   (u16     ),
        glyphIdArray (Vec<u16>) |tape, this| { read_vector!(tape, this.entryCount) },
    }
}

impl Value for CharMapping {
    fn read<T: Tape>(tape: &mut T) -> Result<CharMapping> {
        let position = try!(tape.position());
        let header = match try!(tape.peek::<u16>()) {
            0 => try!(CharMappingHeader::read(tape)),
            _ => raise!("the format of the char-to-glyph mapping header is not supported"),
        };
        let mut records = vec![];
        for _ in 0..header.numTables {
            records.push(try!(CharMappingRecord::read(tape)));
        }
        let mut encodings = vec![];
        for encoding in records.iter() {
            try!(tape.jump(position + encoding.offset as u64));
            encodings.push(match try!(tape.peek::<u16>()) {
                4 => CharMappingEncoding::Format4(try!(Value::read(tape))),
                6 => CharMappingEncoding::Format6(try!(Value::read(tape))),
                _ => unimplemented!(),
            });
        }
        Ok(CharMapping { header: header, records: records, encodings: encodings })
    }
}

impl CharMappingEncoding4 {
    /// Return the mapping.
    pub fn mapping(&self) -> HashMap<u16, u16> {
        let segments = self.segments();
        let mut map = HashMap::new();
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idDelta = self.idDelta[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                let index = if idRangeOffset > 0 {
                    let offset = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as u16;
                    self.glyphIdArray[offset as usize]
                } else {
                    (idDelta + j as i16) as u16
                };
                map.insert(j, index);
            }
        }
        map
    }

    fn array_length(&self) -> Result<usize> {
        let segments = self.segments();
        if segments == 0 {
            raise!("found a char-to-glyph mapping with no segments");
        }
        if self.startCode[segments - 1] != 0xffff || self.endCode[segments - 1] != 0xffff {
            raise!("found a malformed char-to-glyph mapping");
        }
        let mut length = 0;
        for i in 0..(segments - 1) {
            let startCode = self.startCode[i];
            let idRangeOffset = self.idRangeOffset[i];
            for j in startCode..(self.endCode[i] + 1) {
                if idRangeOffset > 0 {
                    let end = (idRangeOffset / 2 + (j - startCode)) - (segments - i) as u16 + 1;
                    if end > length {
                        length = end;
                    }
                }
            }
        }
        Ok(length as usize)
    }

    #[inline]
    fn segments(&self) -> usize {
        self.segCountX2 as usize / 2
    }
}
